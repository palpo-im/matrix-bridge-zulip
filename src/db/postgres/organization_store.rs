use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::error::{DatabaseError, Result};
use crate::db::models::{Organization, OrganizationChangeset};
use crate::db::schema::organizations;
use crate::db::stores::OrganizationStore;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresOrganizationStore {
    pool: Pool,
}

impl PostgresOrganizationStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationStore for PostgresOrganizationStore {
    async fn create(&self, org: Organization) -> Result<Organization> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            diesel::insert_into(organizations::table)
                .values(&org)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get(&self, id: &str) -> Result<Option<Organization>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            organizations::table
                .find(id)
                .first::<Organization>(&mut conn)
                .optional()
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn get_all(&self) -> Result<Vec<Organization>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            organizations::table
                .load::<Organization>(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn update(&self, id: &str, changeset: OrganizationChangeset) -> Result<Organization> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            diesel::update(organizations::table.find(id))
                .set(&changeset)
                .get_result(&mut conn)
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            diesel::delete(organizations::table.find(id))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn set_connected(&self, id: &str, connected: bool) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            diesel::update(organizations::table.find(&id))
                .set(organizations::connected.eq(connected))
                .execute(&mut conn)
                .map(|_| ())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }

    async fn exists(&self, id: &str) -> Result<bool> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            organizations::table
                .find(id)
                .select(organizations::id)
                .first::<String>(&mut conn)
                .optional()
                .map(|opt| opt.is_some())
                .map_err(|e| DatabaseError::Query(e.to_string()))
        })
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?
    }
}
