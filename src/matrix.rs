pub mod event_handler;
pub mod ghost;

pub use self::event_handler::{MatrixEventHandler, MatrixEventProcessor};
pub use self::ghost::GhostUserManager;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use url::Url;

use matrix_bot_sdk::appservice::{Appservice, AppserviceHandler};
use matrix_bot_sdk::client::{MatrixAuth, MatrixClient};
use matrix_bot_sdk::models::CreateRoom;

use crate::config::Config;
use crate::utils::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixEvent {
    pub event_id: Option<String>,
    pub event_type: String,
    pub room_id: String,
    pub sender: String,
    pub state_key: Option<String>,
    pub content: Option<Value>,
    pub timestamp: Option<i64>,
    pub transaction_id: Option<String>,
}

impl MatrixEvent {
    pub fn content_as_str(&self, key: &str) -> Option<&str> {
        self.content
            .as_ref()?
            .get(key)?
            .as_str()
    }

    pub fn content_as_string(&self, key: &str) -> Option<String> {
        self.content_as_str(key).map(|s| s.to_string())
    }

    pub fn msgtype(&self) -> Option<&str> {
        self.content_as_str("msgtype")
    }

    pub fn body(&self) -> Option<&str> {
        self.content_as_str("body")
    }

    pub fn membership(&self) -> Option<&str> {
        self.content_as_str("membership")
    }

    pub fn displayname(&self) -> Option<&str> {
        self.content_as_str("displayname")
    }

    pub fn is_message(&self) -> bool {
        self.event_type == "m.room.message"
    }

    pub fn is_member(&self) -> bool {
        self.event_type == "m.room.member"
    }

    pub fn is_reaction(&self) -> bool {
        self.event_type == "m.reaction"
    }

    pub fn is_redaction(&self) -> bool {
        self.event_type == "m.room.redaction"
    }

    pub fn relates_to_event_id(&self) -> Option<String> {
        let relates_to = self.content.as_ref()?.get("m.relates_to")?;
        
        // Check for reply
        if let Some(reply_to) = relates_to.get("m.in_reply_to") {
            return reply_to.get("event_id")?.as_str().map(|s| s.to_string());
        }
        
        // Check for edit or reaction
        if relates_to.get("rel_type")?.as_str()? == "m.replace" ||
           relates_to.get("rel_type")?.as_str()? == "m.annotation" {
            return relates_to.get("event_id")?.as_str().map(|s| s.to_string());
        }
        
        None
    }

    pub fn reaction_key(&self) -> Option<String> {
        let relates_to = self.content.as_ref()?.get("m.relates_to")?;
        if relates_to.get("rel_type")?.as_str()? == "m.annotation" {
            return relates_to.get("key")?.as_str().map(|s| s.to_string());
        }
        None
    }
}

pub struct BridgeAppserviceHandler {
    processor: Option<Arc<MatrixEventProcessor>>,
}

#[async_trait::async_trait]
impl AppserviceHandler for BridgeAppserviceHandler {
    async fn on_transaction(&self, _txn_id: &str, body: &Value) -> anyhow::Result<()> {
        let Some(processor) = &self.processor else {
            return Ok(());
        };

        if let Some(events) = body.get("events").and_then(|v| v.as_array()) {
            for event in events {
                let matrix_event = MatrixEvent {
                    event_id: event
                        .get("event_id")
                        .and_then(|v| v.as_str())
                        .map(ToOwned::to_owned),
                    event_type: event
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_owned(),
                    room_id: event
                        .get("room_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_owned(),
                    sender: event
                        .get("sender")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_owned(),
                    state_key: event
                        .get("state_key")
                        .and_then(|v| v.as_str())
                        .map(ToOwned::to_owned),
                    content: event.get("content").cloned(),
                    timestamp: event
                        .get("origin_server_ts")
                        .and_then(|v| v.as_i64()),
                    transaction_id: event
                        .get("transaction_id")
                        .and_then(|v| v.as_str())
                        .map(ToOwned::to_owned),
                };

                if let Err(e) = processor.process_event(matrix_event).await {
                    error!("error processing event: {}", e);
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct MatrixAppservice {
    config: Arc<Config>,
    pub appservice: Appservice,
    handler: Arc<RwLock<BridgeAppserviceHandler>>,
}

const GHOST_USER_PREFIX: &str = "_zulip_";

fn ghost_user_localpart(zulip_user_id: i64) -> String {
    format!("{}{}", GHOST_USER_PREFIX, zulip_user_id)
}

fn ghost_user_id(zulip_user_id: i64, domain: &str) -> String {
    format!("@{}:{}", ghost_user_localpart(zulip_user_id), domain)
}

fn is_namespaced_user(user_id: &str) -> bool {
    user_id.contains(&format!("@{}", GHOST_USER_PREFIX))
}

fn build_matrix_message_content(
    body: &str,
    formatted_body: Option<&str>,
    reply_to: Option<&str>,
    edit_of: Option<&str>,
) -> Value {
    let mut content = if let Some(html) = formatted_body {
        json!({
            "msgtype": "m.text",
            "body": body,
            "format": "org.matrix.custom.html",
            "formatted_body": html,
        })
    } else {
        json!({
            "msgtype": "m.text",
            "body": body,
        })
    };

    if let Some(reply_id) = reply_to {
        content["m.relates_to"] = json!({
            "m.in_reply_to": {
                "event_id": reply_id
            }
        });
    }

    if let Some(edit_event_id) = edit_of {
        content["m.new_content"] = if let Some(html) = formatted_body {
            json!({
                "msgtype": "m.text",
                "body": body,
                "format": "org.matrix.custom.html",
                "formatted_body": html,
            })
        } else {
            json!({
                "msgtype": "m.text",
                "body": body,
            })
        };
        content["m.relates_to"] = json!({
            "rel_type": "m.replace",
            "event_id": edit_event_id,
        });
        content["body"] = format!("* {body}");
    }

    content
}

impl MatrixAppservice {
    pub async fn new(config: Arc<Config>) -> Result<Self> {
        info!(
            "initializing matrix appservice for {}",
            config.bridge.domain
        );

        let homeserver_url = Url::parse(&config.bridge.homeserver_url)?;
        let auth = MatrixAuth::new(&config.registration.appservice_token);
        let client = MatrixClient::new(homeserver_url, auth);

        let handler = Arc::new(RwLock::new(BridgeAppserviceHandler { processor: None }));

        struct HandlerWrapper(Arc<RwLock<BridgeAppserviceHandler>>);
        #[async_trait::async_trait]
        impl AppserviceHandler for HandlerWrapper {
            async fn on_transaction(&self, txn_id: &str, body: &Value) -> anyhow::Result<()> {
                self.0.read().await.on_transaction(txn_id, body).await
            }
        }

        let appservice = Appservice::new(
            &config.registration.homeserver_token,
            &config.registration.appservice_token,
            client,
        )
        .with_appservice_id(&config.registration.bridge_id)
        .with_handler(Arc::new(HandlerWrapper(handler.clone())));

        Ok(Self {
            config,
            appservice,
            handler,
        })
    }

    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    pub fn bot_user_id(&self) -> String {
        format!(
            "@{}:{}",
            self.config.registration.sender_localpart, self.config.bridge.domain
        )
    }

    pub fn is_namespaced_user(&self, user_id: &str) -> bool {
        is_namespaced_user(user_id)
    }

    pub fn ghost_user_id(&self, zulip_user_id: i64) -> String {
        ghost_user_id(zulip_user_id, &self.config.bridge.domain)
    }

    pub async fn set_processor(&self, processor: Arc<MatrixEventProcessor>) {
        self.handler.write().await.processor = Some(processor);
    }

    pub async fn start(&self) -> Result<()> {
        info!("matrix appservice starting");
        Ok(())
    }

    pub async fn ensure_bot_joined_room(&self, room_id: &str) -> Result<bool> {
        let bot_user_id = self.bot_user_id();
        let membership = self
            .appservice
            .client
            .get_room_state_event(room_id, "m.room.member", &bot_user_id)
            .await
            .ok()
            .and_then(|state| {
                state
                    .get("membership")
                    .and_then(|value| value.as_str())
                    .map(ToOwned::to_owned)
            });

        match membership.as_deref() {
            Some("join") => Ok(false),
            Some("invite") => {
                let joined = self.appservice.client.join_room(room_id).await?;
                info!(
                    "auto-joined invited room {} as {}",
                    joined, bot_user_id
                );
                Ok(true)
            }
            _ => {
                debug!(
                    "bot not in room or cannot join room_id={} bot_user={}",
                    room_id, bot_user_id
                );
                Ok(false)
            }
        }
    }

    pub async fn create_room(
        &self,
        name: &str,
        alias_localpart: Option<&str>,
        topic: Option<&str>,
        is_public: bool,
    ) -> Result<String> {
        let visibility = if is_public {
            Some("public".to_string())
        } else {
            Some("private".to_string())
        };

        let opt = CreateRoom {
            visibility,
            room_alias_name: alias_localpart.map(|s| s.to_string()),
            name: Some(name.to_owned()),
            topic: topic.map(ToOwned::to_owned),
            ..Default::default()
        };

        let room_id = self.appservice.client.create_room(&opt).await?;
        Ok(room_id)
    }

    pub async fn send_message(
        &self,
        room_id: &str,
        sender: &str,
        content: &str,
        formatted_content: Option<&str>,
    ) -> Result<String> {
        let matrix_content = build_matrix_message_content(content, formatted_content, None, None);
        
        let client = self.get_ghost_client(sender).await?;
        let event_id = client.send_room_message(room_id, &matrix_content).await?;
        
        Ok(event_id)
    }

    pub async fn send_message_with_reply(
        &self,
        room_id: &str,
        sender: &str,
        content: &str,
        formatted_content: Option<&str>,
        reply_to: &str,
    ) -> Result<String> {
        let matrix_content =
            build_matrix_message_content(content, formatted_content, Some(reply_to), None);

        let client = self.get_ghost_client(sender).await?;
        let event_id = client.send_room_message(room_id, &matrix_content).await?;

        Ok(event_id)
    }

    pub async fn send_message_edit(
        &self,
        room_id: &str,
        sender: &str,
        content: &str,
        formatted_content: Option<&str>,
        edit_of: &str,
    ) -> Result<String> {
        let matrix_content =
            build_matrix_message_content(content, formatted_content, None, Some(edit_of));

        let client = self.get_ghost_client(sender).await?;
        let event_id = client.send_room_message(room_id, &matrix_content).await?;

        Ok(event_id)
    }

    pub async fn send_reaction(
        &self,
        room_id: &str,
        sender: &str,
        event_id: &str,
        key: &str,
    ) -> Result<String> {
        let content = json!({
            "m.relates_to": {
                "rel_type": "m.annotation",
                "event_id": event_id,
                "key": key
            }
        });

        let client = self.get_ghost_client(sender).await?;
        let reaction_event_id = client
            .send_room_event(room_id, "m.reaction", &content)
            .await?;

        Ok(reaction_event_id)
    }

    pub async fn redact_event(
        &self,
        room_id: &str,
        sender: &str,
        event_id: &str,
        reason: Option<&str>,
    ) -> Result<()> {
        let client = self.get_ghost_client(sender).await?;
        client
            .redact_room_event(room_id, event_id, reason)
            .await?;
        Ok(())
    }

    pub async fn set_room_name(&self, room_id: &str, name: &str) -> Result<()> {
        self.appservice.client.set_room_name(room_id, name).await?;
        Ok(())
    }

    pub async fn set_room_topic(&self, room_id: &str, topic: &str) -> Result<()> {
        self.appservice.client.set_room_topic(room_id, topic).await?;
        Ok(())
    }

    pub async fn get_room_members(&self, room_id: &str) -> Result<Vec<String>> {
        let members = self.appservice.client.get_room_members(room_id).await?;
        Ok(members)
    }

    pub async fn invite_user(&self, room_id: &str, user_id: &str) -> Result<()> {
        self.appservice.client.invite_user(room_id, user_id).await?;
        Ok(())
    }

    pub async fn kick_user(&self, room_id: &str, user_id: &str, reason: Option<&str>) -> Result<()> {
        self.appservice
            .client
            .kick_user(room_id, user_id, reason)
            .await?;
        Ok(())
    }

    pub async fn leave_room(&self, room_id: &str) -> Result<()> {
        self.appservice.client.leave_room(room_id).await?;
        Ok(())
    }

    async fn get_ghost_client(&self, user_id: &str) -> Result<MatrixClient> {
        let client = self.appservice.client.clone();
        client.impersonate_user_id(Some(user_id), None::<&str>).await;
        Ok(client)
    }
}
