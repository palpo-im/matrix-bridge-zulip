use async_trait::async_trait;

use crate::db::error::Result;
use crate::db::models::{NewUserMapping, UserMapping, UserMappingChangeset};

#[async_trait]
pub trait UserStore: Send + Sync {
    async fn create(&self, user: NewUserMapping) -> Result<UserMapping>;
    
    async fn get(&self, id: i64) -> Result<Option<UserMapping>>;
    
    async fn get_by_matrix_user(&self, matrix_user_id: &str) -> Result<Option<UserMapping>>;
    
    async fn get_by_zulip_user(&self, zulip_user_id: i64) -> Result<Option<UserMapping>>;
    
    async fn update(&self, id: i64, changeset: UserMappingChangeset) -> Result<UserMapping>;
    
    async fn update_by_matrix_user(
        &self,
        matrix_user_id: &str,
        changeset: UserMappingChangeset,
    ) -> Result<UserMapping>;
    
    async fn delete(&self, id: i64) -> Result<()>;
    
    async fn delete_by_matrix_user(&self, matrix_user_id: &str) -> Result<()>;
    
    async fn exists(&self, matrix_user_id: &str) -> Result<bool>;
}
