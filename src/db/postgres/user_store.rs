use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::error::{DatabaseError, Result};
use crate::db::models::{NewUserMapping, UserMapping, UserMappingChangeset};
use crate::db::schema::user_mappings;
use crate::db::stores::UserStore;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresUserStore {
    pool: Pool,
}

impl PostgresUserStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserStore for PostgresUserStore {
    async fn create(&self, user: NewUserMapping) -> Result<UserMapping> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::insert_into(user_mappings::table)
                .values(&user)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get(&self, id: i64) -> Result<Option<UserMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            user_mappings::table
                .find(id)
                .first::<UserMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_matrix_user(&self, matrix_user_id: &str) -> Result<Option<UserMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_user_id = matrix_user_id.to_string();
        tokio::task::spawn_blocking(move || {
            user_mappings::table
                .filter(user_mappings::matrix_user_id.eq(matrix_user_id))
                .first::<UserMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_by_zulip_user(&self, zulip_user_id: i64) -> Result<Option<UserMapping>> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            user_mappings::table
                .filter(user_mappings::zulip_user_id.eq(zulip_user_id))
                .first::<UserMapping>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn update(&self, id: i64, changeset: UserMappingChangeset) -> Result<UserMapping> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::update(user_mappings::table.find(id))
                .set(&changeset)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn update_by_matrix_user(
        &self,
        matrix_user_id: &str,
        changeset: UserMappingChangeset,
    ) -> Result<UserMapping> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_user_id = matrix_user_id.to_string();
        tokio::task::spawn_blocking(move || {
            diesel::update(user_mappings::table.filter(user_mappings::matrix_user_id.eq(matrix_user_id)))
                .set(&changeset)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(user_mappings::table.find(id))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete_by_matrix_user(&self, matrix_user_id: &str) -> Result<()> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_user_id = matrix_user_id.to_string();
        tokio::task::spawn_blocking(move || {
            diesel::delete(user_mappings::table.filter(user_mappings::matrix_user_id.eq(matrix_user_id)))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists(&self, matrix_user_id: &str) -> Result<bool> {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::Connection(e.to_string()))?;
        let matrix_user_id = matrix_user_id.to_string();
        tokio::task::spawn_blocking(move || {
            user_mappings::table
                .filter(user_mappings::matrix_user_id.eq(matrix_user_id))
                .select(user_mappings::id)
                .first::<i64>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }
}
