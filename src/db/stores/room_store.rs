use async_trait::async_trait;

use crate::db::error::Result;
use crate::db::models::{NewRoomMapping, RoomMapping, RoomType};

#[async_trait]
pub trait RoomStore: Send + Sync {
    async fn create(&self, room: NewRoomMapping) -> Result<RoomMapping>;
    
    async fn get(&self, id: i64) -> Result<Option<RoomMapping>>;
    
    async fn get_by_matrix_room(&self, matrix_room_id: &str) -> Result<Option<RoomMapping>>;
    
    async fn get_by_zulip_stream(
        &self,
        organization_id: &str,
        zulip_stream_id: i64,
    ) -> Result<Option<RoomMapping>>;
    
    async fn get_by_organization(&self, organization_id: &str) -> Result<Vec<RoomMapping>>;
    
    async fn get_by_type(&self, organization_id: &str, room_type: RoomType) -> Result<Vec<RoomMapping>>;
    
    async fn delete(&self, id: i64) -> Result<()>;
    
    async fn delete_by_matrix_room(&self, matrix_room_id: &str) -> Result<()>;
    
    async fn exists(&self, matrix_room_id: &str) -> Result<bool>;
}
