use async_trait::async_trait;

use crate::db::error::Result;
use crate::db::models::{NewReactionMapping, ReactionMapping};

#[async_trait]
pub trait ReactionStore: Send + Sync {
    async fn create(&self, reaction: NewReactionMapping) -> Result<ReactionMapping>;
    
    async fn get(&self, id: i64) -> Result<Option<ReactionMapping>>;
    
    async fn get_by_matrix_reaction(&self, matrix_reaction_event_id: &str) -> Result<Option<ReactionMapping>>;
    
    async fn get_by_zulip_reaction(&self, zulip_reaction_id: i64) -> Result<Option<ReactionMapping>>;
    
    async fn get_by_zulip_message(&self, zulip_message_id: i64) -> Result<Vec<ReactionMapping>>;
    
    async fn delete(&self, id: i64) -> Result<()>;
    
    async fn delete_by_matrix_reaction(&self, matrix_reaction_event_id: &str) -> Result<()>;
    
    async fn delete_by_zulip_reaction(&self, zulip_reaction_id: i64) -> Result<()>;
    
    async fn exists_by_matrix_reaction(&self, matrix_reaction_event_id: &str) -> Result<bool>;
    
    async fn exists_by_zulip_reaction(&self, zulip_reaction_id: i64) -> Result<bool>;
}
