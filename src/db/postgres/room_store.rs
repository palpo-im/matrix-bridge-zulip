use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::error::{DatabaseError, Result};
use crate::db::models::{NewRoomMapping, RoomMapping, RoomType};
use crate::db::schema::room_mappings;
use crate::db::stores::RoomStore;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresRoomStore {
    pool: Pool,
}

impl PostgresRoomStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoomStore for PostgresRoomStore {
    async fn create(&self, room: NewRoomMapping) -> Result<RoomMapping> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::insert_into(room_mappings::table)
                .values(&room)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get(&self, id: i64) -> Result<Option<RoomMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            room_mappings::table
                .find(id)
                .first::<RoomMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_matrix_room(&self, matrix_room_id: &str) -> Result<Option<RoomMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_room_id = matrix_room_id.to_string();
        tokio::task::spawn_blocking(move || {
            room_mappings::table
                .filter(room_mappings::matrix_room_id.eq(matrix_room_id))
                .first::<RoomMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_zulip_stream(&self, organization_id: &str, zulip_stream_id: i64) -> Result<Option<RoomMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let organization_id = organization_id.to_string();
        tokio::task::spawn_blocking(move || {
            room_mappings::table
                .filter(room_mappings::organization_id.eq(organization_id))
                .filter(room_mappings::zulip_stream_id.eq(zulip_stream_id))
                .first::<RoomMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_organization(&self, organization_id: &str) -> Result<Vec<RoomMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let organization_id = organization_id.to_string();
        tokio::task::spawn_blocking(move || {
            room_mappings::table
                .filter(room_mappings::organization_id.eq(organization_id))
                .load::<RoomMapping>(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_type(&self, organization_id: &str, room_type: RoomType) -> Result<Vec<RoomMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let organization_id = organization_id.to_string();
        let room_type_str = room_type.to_string();
        tokio::task::spawn_blocking(move || {
            room_mappings::table
                .filter(room_mappings::organization_id.eq(organization_id))
                .filter(room_mappings::room_type.eq(room_type_str))
                .load::<RoomMapping>(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(room_mappings::table.find(id))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete_by_matrix_room(&self, matrix_room_id: &str) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_room_id = matrix_room_id.to_string();
        tokio::task::spawn_blocking(move || {
            diesel::delete(room_mappings::table.filter(room_mappings::matrix_room_id.eq(matrix_room_id)))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists(&self, matrix_room_id: &str) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_room_id = matrix_room_id.to_string();
        tokio::task::spawn_blocking(move || {
            room_mappings::table
                .filter(room_mappings::matrix_room_id.eq(matrix_room_id))
                .select(room_mappings::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }
}
