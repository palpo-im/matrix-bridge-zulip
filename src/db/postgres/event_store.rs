use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::error::{DatabaseError, Result};
use crate::db::models::{NewProcessedEvent, ProcessedEvent};
use crate::db::schema::processed_events;
use crate::db::stores::EventStore;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresEventStore {
    pool: Pool,
}

impl PostgresEventStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventStore for PostgresEventStore {
    async fn create(&self, event: NewProcessedEvent) -> Result<ProcessedEvent> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::insert_into(processed_events::table)
                .values(&event)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get(&self, id: i64) -> Result<Option<ProcessedEvent>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            processed_events::table
                .find(id)
                .first::<ProcessedEvent>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_event_id(&self, event_id: &str) -> Result<Option<ProcessedEvent>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let event_id = event_id.to_string();
        tokio::task::spawn_blocking(move || {
            processed_events::table
                .filter(processed_events::event_id.eq(event_id))
                .first::<ProcessedEvent>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists(&self, event_id: &str) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let event_id = event_id.to_string();
        tokio::task::spawn_blocking(move || {
            processed_events::table
                .filter(processed_events::event_id.eq(event_id))
                .select(processed_events::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete_old_events(&self, before: DateTime<Utc>) -> Result<usize> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(processed_events::table.filter(processed_events::processed_at.lt(before)))
                .execute(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }
}
