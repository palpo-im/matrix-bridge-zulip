use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::error::{DatabaseError, Result};
use crate::db::models::{NewReactionMapping, ReactionMapping};
use crate::db::schema::reaction_mappings;
use crate::db::stores::ReactionStore;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresReactionStore {
    pool: Pool,
}

impl PostgresReactionStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReactionStore for PostgresReactionStore {
    async fn create(&self, reaction: NewReactionMapping) -> Result<ReactionMapping> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::insert_into(reaction_mappings::table)
                .values(&reaction)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get(&self, id: i64) -> Result<Option<ReactionMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            reaction_mappings::table
                .find(id)
                .first::<ReactionMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_matrix_reaction(&self, matrix_reaction_event_id: &str) -> Result<Option<ReactionMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_reaction_event_id = matrix_reaction_event_id.to_string();
        tokio::task::spawn_blocking(move || {
            reaction_mappings::table
                .filter(reaction_mappings::matrix_reaction_event_id.eq(matrix_reaction_event_id))
                .first::<ReactionMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_zulip_reaction(&self, zulip_reaction_id: i64) -> Result<Option<ReactionMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            reaction_mappings::table
                .filter(reaction_mappings::zulip_reaction_id.eq(zulip_reaction_id))
                .first::<ReactionMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_zulip_message(&self, zulip_message_id: i64) -> Result<Vec<ReactionMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            reaction_mappings::table
                .filter(reaction_mappings::zulip_message_id.eq(zulip_message_id))
                .load::<ReactionMapping>(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(reaction_mappings::table.find(id))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete_by_matrix_reaction(&self, matrix_reaction_event_id: &str) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_reaction_event_id = matrix_reaction_event_id.to_string();
        tokio::task::spawn_blocking(move || {
            diesel::delete(
                reaction_mappings::table
                    .filter(reaction_mappings::matrix_reaction_event_id.eq(matrix_reaction_event_id)),
            )
            .execute(&mut conn)
            .map(|_| ())
            .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete_by_zulip_reaction(&self, zulip_reaction_id: i64) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(reaction_mappings::table.filter(reaction_mappings::zulip_reaction_id.eq(zulip_reaction_id)))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists_by_matrix_reaction(&self, matrix_reaction_event_id: &str) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_reaction_event_id = matrix_reaction_event_id.to_string();
        tokio::task::spawn_blocking(move || {
            reaction_mappings::table
                .filter(reaction_mappings::matrix_reaction_event_id.eq(matrix_reaction_event_id))
                .select(reaction_mappings::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists_by_zulip_reaction(&self, zulip_reaction_id: i64) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            reaction_mappings::table
                .filter(reaction_mappings::zulip_reaction_id.eq(zulip_reaction_id))
                .select(reaction_mappings::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }
}
