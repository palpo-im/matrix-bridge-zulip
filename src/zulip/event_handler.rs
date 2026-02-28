use std::sync::Arc;

use async_trait::async_trait;
use tracing::{debug, info, warn};

use super::ZulipEvent;
use crate::utils::Result;

#[async_trait]
pub trait ZulipEventHandler: Send + Sync {
    async fn handle_message(&self, event: &ZulipEvent) -> Result<()>;
    async fn handle_reaction(&self, event: &ZulipEvent) -> Result<()>;
    async fn handle_update_message(&self, event: &ZulipEvent) -> Result<()>;
    async fn handle_delete_message(&self, event: &ZulipEvent) -> Result<()>;
    async fn handle_subscription(&self, event: &ZulipEvent) -> Result<()>;
    async fn handle_realm_user(&self, event: &ZulipEvent) -> Result<()>;
}

pub struct DefaultZulipEventHandler;

#[async_trait]
impl ZulipEventHandler for DefaultZulipEventHandler {
    async fn handle_message(&self, event: &ZulipEvent) -> Result<()> {
        if let Some(msg) = &event.message {
            debug!(
                "Zulip message {} from user {} in {}",
                msg.id,
                msg.sender_id,
                msg.stream_id.map(|s| s.to_string()).unwrap_or_else(|| "DM".to_string())
            );
        }
        Ok(())
    }

    async fn handle_reaction(&self, event: &ZulipEvent) -> Result<()> {
        if let (Some(msg_id), Some(emoji)) = (event.message_id, &event.emoji_name) {
            debug!("Zulip reaction {} on message {}", emoji, msg_id);
        }
        Ok(())
    }

    async fn handle_update_message(&self, event: &ZulipEvent) -> Result<()> {
        if let Some(msg_id) = event.message_id {
            debug!("Zulip message {} updated", msg_id);
        }
        Ok(())
    }

    async fn handle_delete_message(&self, event: &ZulipEvent) -> Result<()> {
        if let Some(msg_id) = event.message_id {
            debug!("Zulip message {} deleted", msg_id);
        }
        Ok(())
    }

    async fn handle_subscription(&self, event: &ZulipEvent) -> Result<()> {
        if let Some(stream_id) = event.stream_id {
            debug!("Zulip subscription changed for stream {}", stream_id);
        }
        Ok(())
    }

    async fn handle_realm_user(&self, event: &ZulipEvent) -> Result<()> {
        if let Some(user_id) = event.user_id {
            debug!("Zulip realm user {} changed", user_id);
        }
        Ok(())
    }
}

pub struct ZulipEventProcessor {
    handler: Arc<dyn ZulipEventHandler>,
    processed_events: std::collections::HashSet<i64>,
    max_processed_events: usize,
}

impl ZulipEventProcessor {
    pub fn new(handler: Arc<dyn ZulipEventHandler>) -> Self {
        Self {
            handler,
            processed_events: std::collections::HashSet::new(),
            max_processed_events: 10000,
        }
    }

    pub async fn process_event(&mut self, event: ZulipEvent) -> Result<()> {
        let event_id = event.id.unwrap_or(-1);
        
        if event_id >= 0 && self.processed_events.contains(&event_id) {
            debug!("Skipping already processed event {}", event_id);
            return Ok(());
        }

        debug!("Processing Zulip event type={}", event.event_type);

        match event.event_type.as_str() {
            "message" => {
                self.handler.handle_message(&event).await?;
            }
            "reaction" => {
                self.handler.handle_reaction(&event).await?;
            }
            "update_message" => {
                self.handler.handle_update_message(&event).await?;
            }
            "delete_message" => {
                self.handler.handle_delete_message(&event).await?;
            }
            "subscription" => {
                self.handler.handle_subscription(&event).await?;
            }
            "realm_user" => {
                self.handler.handle_realm_user(&event).await?;
            }
            _ => {
                debug!("Ignoring unhandled event type: {}", event.event_type);
            }
        }

        if event_id >= 0 {
            self.processed_events.insert(event_id);
            
            if self.processed_events.len() > self.max_processed_events {
                self.processed_events.clear();
            }
        }

        Ok(())
    }
}
