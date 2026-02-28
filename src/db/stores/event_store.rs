use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::db::error::Result;
use crate::db::models::{NewProcessedEvent, ProcessedEvent};

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn create(&self, event: NewProcessedEvent) -> Result<ProcessedEvent>;
    
    async fn get(&self, id: i64) -> Result<Option<ProcessedEvent>>;
    
    async fn get_by_event_id(&self, event_id: &str) -> Result<Option<ProcessedEvent>>;
    
    async fn exists(&self, event_id: &str) -> Result<bool>;
    
    async fn delete_old_events(&self, before: DateTime<Utc>) -> Result<usize>;
    
    async fn cleanup(&self, retention_days: i32) -> Result<usize> {
        let before = Utc::now() - chrono::Duration::days(retention_days as i64);
        self.delete_old_events(before).await
    }
}
