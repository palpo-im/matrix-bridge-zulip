use async_trait::async_trait;

use crate::db::error::Result;
use crate::db::models::{MessageMapping, NewMessageMapping};

#[async_trait]
pub trait MessageStore: Send + Sync {
    async fn create(&self, message: NewMessageMapping) -> Result<MessageMapping>;
    
    async fn get(&self, id: i64) -> Result<Option<MessageMapping>>;
    
    async fn get_by_matrix_event(&self, matrix_event_id: &str) -> Result<Option<MessageMapping>>;
    
    async fn get_by_zulip_message(&self, zulip_message_id: i64) -> Result<Option<MessageMapping>>;
    
    async fn get_by_matrix_room(&self, matrix_room_id: &str, limit: i64) -> Result<Vec<MessageMapping>>;
    
    async fn delete(&self, id: i64) -> Result<()>;
    
    async fn delete_by_matrix_event(&self, matrix_event_id: &str) -> Result<()>;
    
    async fn exists_by_matrix_event(&self, matrix_event_id: &str) -> Result<bool>;
    
    async fn exists_by_zulip_message(&self, zulip_message_id: i64) -> Result<bool>;
}
