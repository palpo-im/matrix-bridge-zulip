use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{debug, info, warn};

use super::MatrixEvent;
use crate::utils::Result;

const DEFAULT_AGE_LIMIT_MS: i64 = 900_000;

#[async_trait]
pub trait MatrixEventHandler: Send + Sync {
    async fn handle_room_message(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_room_member(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_room_redaction(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_reaction(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_room_encryption(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_room_name(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_room_topic(&self, event: &MatrixEvent) -> Result<()>;
    async fn handle_room_avatar(&self, event: &MatrixEvent) -> Result<()>;
}

pub struct DefaultMatrixEventHandler;

#[async_trait]
impl MatrixEventHandler for DefaultMatrixEventHandler {
    async fn handle_room_message(&self, event: &MatrixEvent) -> Result<()> {
        debug!(
            "received message event in room {} from {}",
            event.room_id, event.sender
        );
        Ok(())
    }

    async fn handle_room_member(&self, event: &MatrixEvent) -> Result<()> {
        if let Some(membership) = event.membership() {
            debug!(
                "member {} changed membership to {} in room {}",
                event.sender, membership, event.room_id
            );
        }
        Ok(())
    }

    async fn handle_room_redaction(&self, event: &MatrixEvent) -> Result<()> {
        debug!(
            "redaction event in room {} from {}",
            event.room_id, event.sender
        );
        Ok(())
    }

    async fn handle_reaction(&self, event: &MatrixEvent) -> Result<()> {
        if let Some(key) = event.reaction_key() {
            debug!(
                "reaction {} in room {} from {}",
                key, event.room_id, event.sender
            );
        }
        Ok(())
    }

    async fn handle_room_encryption(&self, event: &MatrixEvent) -> Result<()> {
        warn!(
            "room {} has been marked as encrypted, bridge may not work correctly",
            event.room_id
        );
        Ok(())
    }

    async fn handle_room_name(&self, event: &MatrixEvent) -> Result<()> {
        debug!("room {} name changed", event.room_id);
        Ok(())
    }

    async fn handle_room_topic(&self, event: &MatrixEvent) -> Result<()> {
        debug!("room {} topic changed", event.room_id);
        Ok(())
    }

    async fn handle_room_avatar(&self, event: &MatrixEvent) -> Result<()> {
        debug!("room {} avatar changed", event.room_id);
        Ok(())
    }
}

pub struct MatrixEventProcessor {
    event_handler: Arc<dyn MatrixEventHandler>,
    age_limit_ms: i64,
}

impl MatrixEventProcessor {
    pub fn new(event_handler: Arc<dyn MatrixEventHandler>) -> Self {
        Self {
            event_handler,
            age_limit_ms: DEFAULT_AGE_LIMIT_MS,
        }
    }

    pub fn with_age_limit(event_handler: Arc<dyn MatrixEventHandler>, age_limit_ms: u64) -> Self {
        let age_limit_ms = std::cmp::min(age_limit_ms, i64::MAX as u64) as i64;
        Self {
            event_handler,
            age_limit_ms,
        }
    }

    fn check_event_age(event: &MatrixEvent, age_limit_ms: i64) -> bool {
        if age_limit_ms <= 0 {
            return true;
        }

        if let Some(ts) = event.timestamp {
            let now = Utc::now().timestamp_millis();
            if ts > now {
                debug!(
                    "event timestamp is in the future, allowing event_id={:?}",
                    event.event_id
                );
                return true;
            }
            let age = now - ts;
            if age > age_limit_ms {
                info!(
                    "skipping event due to age {}ms > {}ms event_id={:?} room_id={} type={}",
                    age, age_limit_ms, event.event_id, event.room_id, event.event_type
                );
                return false;
            }
        }
        true
    }

    pub async fn process_event(&self, event: MatrixEvent) -> Result<()> {
        if !Self::check_event_age(&event, self.age_limit_ms) {
            return Ok(());
        }

        debug!(
            "processing event type={} room={} sender={}",
            event.event_type, event.room_id, event.sender
        );

        match event.event_type.as_str() {
            "m.room.message" => {
                self.event_handler.handle_room_message(&event).await?;
            }
            "m.room.member" => {
                self.event_handler.handle_room_member(&event).await?;
            }
            "m.room.redaction" => {
                self.event_handler.handle_room_redaction(&event).await?;
            }
            "m.reaction" => {
                self.event_handler.handle_reaction(&event).await?;
            }
            "m.room.encryption" => {
                self.event_handler.handle_room_encryption(&event).await?;
            }
            "m.room.name" => {
                self.event_handler.handle_room_name(&event).await?;
            }
            "m.room.topic" => {
                self.event_handler.handle_room_topic(&event).await?;
            }
            "m.room.avatar" => {
                self.event_handler.handle_room_avatar(&event).await?;
            }
            _ => {
                debug!("ignoring event type: {}", event.event_type);
            }
        }

        Ok(())
    }
}
