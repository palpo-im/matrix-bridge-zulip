use std::sync::Arc;

use lru::LruCache;
use parking_lot::Mutex;
use tracing::{debug, info, warn};

use super::MatrixAppservice;
use crate::db::stores::UserStore;
use crate::db::models::{NewUserMapping, UserMapping, UserMappingChangeset};
use crate::utils::Result;

const GHOST_USER_PREFIX: &str = "_zulip_";

pub struct GhostUserManager {
    appservice: Arc<MatrixAppservice>,
    user_store: Arc<dyn UserStore>,
    cache: Mutex<LruCache<i64, GhostUserInfo>>,
}

#[derive(Clone, Debug)]
pub struct GhostUserInfo {
    pub zulip_user_id: i64,
    pub matrix_user_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl GhostUserManager {
    pub fn new(appservice: Arc<MatrixAppservice>, user_store: Arc<dyn UserStore>) -> Self {
        Self {
            appservice,
            user_store,
            cache: Mutex::new(LruCache::new(std::num::NonZeroUsize::new(1000).unwrap())),
        }
    }

    pub fn ghost_user_id(&self, zulip_user_id: i64) -> String {
        self.appservice.ghost_user_id(zulip_user_id)
    }

    pub fn is_ghost_user(&self, matrix_user_id: &str) -> bool {
        self.appservice.is_namespaced_user(matrix_user_id)
    }

    pub async fn get_or_create_ghost(
        &self,
        zulip_user_id: i64,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
        is_bot: bool,
    ) -> Result<GhostUserInfo> {
        if let Some(cached) = self.cache.lock().get(&zulip_user_id).cloned() {
            return Ok(cached);
        }

        let matrix_user_id = self.ghost_user_id(zulip_user_id);

        if let Some(existing) = self.user_store.get_by_zulip_user(zulip_user_id).await? {
            let info = GhostUserInfo {
                zulip_user_id,
                matrix_user_id: existing.matrix_user_id.clone(),
                display_name: existing.display_name.clone(),
                avatar_url: existing.avatar_url.clone(),
            };
            
            self.cache.lock().put(zulip_user_id, info.clone());
            return Ok(info);
        }

        let localpart = format!("{}{}", GHOST_USER_PREFIX, zulip_user_id);
        
        let ghost_client = self.appservice.appservice.client.clone();
        ghost_client
            .impersonate_user_id(Some(&matrix_user_id), None::<&str>)
            .await;

        let register_result = ghost_client
            .password_register(&localpart, "", display_name)
            .await;

        if register_result.is_err() {
            debug!(
                "ghost user {} may already exist, attempting login",
                matrix_user_id
            );
        }

        if let Some(name) = display_name {
            let _ = ghost_client.set_display_name(name).await;
        }

        let new_mapping = NewUserMapping {
            matrix_user_id: matrix_user_id.clone(),
            zulip_user_id,
            zulip_email: None,
            display_name: display_name.map(|s| s.to_string()),
            avatar_url: avatar_url.map(|s| s.to_string()),
            is_bot,
        };

        self.user_store.create(new_mapping).await?;

        let info = GhostUserInfo {
            zulip_user_id,
            matrix_user_id,
            display_name: display_name.map(|s| s.to_string()),
            avatar_url: avatar_url.map(|s| s.to_string()),
        };

        self.cache.lock().put(zulip_user_id, info.clone());

        info!(
            "created ghost user for zulip user {}: {}",
            zulip_user_id, info.matrix_user_id
        );

        Ok(info)
    }

    pub async fn update_ghost_profile(
        &self,
        zulip_user_id: i64,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<()> {
        let matrix_user_id = self.ghost_user_id(zulip_user_id);

        let ghost_client = self.appservice.appservice.client.clone();
        ghost_client
            .impersonate_user_id(Some(&matrix_user_id), None::<&str>)
            .await;

        if let Some(name) = display_name {
            ghost_client.set_display_name(name).await?;
        }

        if let Some(_url) = avatar_url {
            // TODO: Implement avatar upload if needed
        }

        let changeset = UserMappingChangeset {
            zulip_email: None,
            display_name: display_name.map(|s| s.to_string()),
            avatar_url: avatar_url.map(|s| s.to_string()),
            is_bot: false,
            updated_at: chrono::Utc::now(),
        };

        self.user_store
            .update_by_matrix_user(&matrix_user_id, changeset)
            .await?;

        if let Some(mut cached) = self.cache.lock().get_mut(&zulip_user_id) {
            if let Some(name) = display_name {
                cached.display_name = Some(name.to_string());
            }
            if let Some(url) = avatar_url {
                cached.avatar_url = Some(url.to_string());
            }
        }

        Ok(())
    }

    pub async fn ensure_ghost_in_room(
        &self,
        zulip_user_id: i64,
        room_id: &str,
    ) -> Result<()> {
        let matrix_user_id = self.ghost_user_id(zulip_user_id);

        let members = self.appservice.get_room_members(room_id).await?;
        if members.contains(&matrix_user_id) {
            return Ok(());
        }

        self.appservice.invite_user(room_id, &matrix_user_id).await?;

        debug!(
            "invited ghost user {} to room {}",
            matrix_user_id, room_id
        );

        Ok(())
    }

    pub async fn remove_ghost_from_room(
        &self,
        zulip_user_id: i64,
        room_id: &str,
    ) -> Result<()> {
        let matrix_user_id = self.ghost_user_id(zulip_user_id);

        let ghost_client = self.appservice.appservice.client.clone();
        ghost_client
            .impersonate_user_id(Some(&matrix_user_id), None::<&str>)
            .await;

        ghost_client.leave_room(room_id).await?;

        debug!(
            "ghost user {} left room {}",
            matrix_user_id, room_id
        );

        Ok(())
    }

    pub async fn get_matrix_user_id(&self, zulip_user_id: i64) -> Result<String> {
        if let Some(cached) = self.cache.lock().get(&zulip_user_id) {
            return Ok(cached.matrix_user_id.clone());
        }

        if let Some(mapping) = self.user_store.get_by_zulip_user(zulip_user_id).await? {
            return Ok(mapping.matrix_user_id);
        }

        Ok(self.ghost_user_id(zulip_user_id))
    }

    pub async fn get_zulip_user_id(&self, matrix_user_id: &str) -> Result<Option<i64>> {
        if !self.is_ghost_user(matrix_user_id) {
            return Ok(None);
        }

        if let Some(mapping) = self.user_store.get_by_matrix_user(matrix_user_id).await? {
            return Ok(Some(mapping.zulip_user_id));
        }

        let prefix = format!("@{}", GHOST_USER_PREFIX);
        if let Some(rest) = matrix_user_id.strip_prefix(&prefix) {
            if let Some(localpart) = rest.split(':').next() {
                if let Ok(id) = localpart.parse::<i64>() {
                    return Ok(Some(id));
                }
            }
        }

        Ok(None)
    }

    pub fn clear_cache(&self) {
        self.cache.lock().clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache.lock().len()
    }
}
