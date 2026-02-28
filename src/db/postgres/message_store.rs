use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::error::{DatabaseError, Result};
use crate::db::models::{MessageMapping, NewMessageMapping};
use crate::db::schema::message_mappings;
use crate::db::stores::MessageStore;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresMessageStore {
    pool: Pool,
}

impl PostgresMessageStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageStore for PostgresMessageStore {
    async fn create(&self, message: NewMessageMapping) -> Result<MessageMapping> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::insert_into(message_mappings::table)
                .values(&message)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get(&self, id: i64) -> Result<Option<MessageMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            message_mappings::table
                .find(id)
                .first::<MessageMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_matrix_event(&self, matrix_event_id: &str) -> Result<Option<MessageMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_event_id = matrix_event_id.to_string();
        tokio::task::spawn_blocking(move || {
            message_mappings::table
                .filter(message_mappings::matrix_event_id.eq(matrix_event_id))
                .first::<MessageMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_zulip_message(&self, zulip_message_id: i64) -> Result<Option<MessageMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            message_mappings::table
                .filter(message_mappings::zulip_message_id.eq(zulip_message_id))
                .first::<MessageMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_matrix_room(&self, matrix_room_id: &str, limit: i64) -> Result<Vec<MessageMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_room_id = matrix_room_id.to_string();
        tokio::task::spawn_blocking(move || {
            message_mappings::table
                .filter(message_mappings::matrix_room_id.eq(matrix_room_id))
                .order(message_mappings::created_at.desc())
                .limit(limit)
                .load::<MessageMapping>(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(message_mappings::table.find(id))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete_by_matrix_event(&self, matrix_event_id: &str) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_event_id = matrix_event_id.to_string();
        tokio::task::spawn_blocking(move || {
            diesel::delete(message_mappings::table.filter(message_mappings::matrix_event_id.eq(matrix_event_id)))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists_by_matrix_event(&self, matrix_event_id: &str) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_event_id = matrix_event_id.to_string();
        tokio::task::spawn_blocking(move || {
            message_mappings::table
                .filter(message_mappings::matrix_event_id.eq(matrix_event_id))
                .select(message_mappings::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists_by_zulip_message(&self, zulip_message_id: i64) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            message_mappings::table
                .filter(message_mappings::zulip_message_id.eq(zulip_message_id))
                .select(message_mappings::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }
}
