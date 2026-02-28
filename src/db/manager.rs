use std::sync::Arc;

#[cfg(feature = "postgres")]
use diesel::pg::PgConnection;
#[cfg(feature = "postgres")]
use diesel::r2d2::{self, ConnectionManager};

use crate::config::DatabaseConfig;
use crate::db::error::{DatabaseError, Result};
use crate::db::stores::{
    EventStore, MessageStore, OrganizationStore, ReactionStore, RoomStore, UserStore,
};

#[cfg(feature = "postgres")]
use crate::db::postgres::{
    PostgresEventStore, PostgresMessageStore, PostgresOrganizationStore, PostgresReactionStore,
    PostgresRoomStore, PostgresUserStore,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbType {
    Postgres,
    Sqlite,
    Mysql,
}

impl From<crate::config::DbType> for DbType {
    fn from(value: crate::config::DbType) -> Self {
        match value {
            crate::config::DbType::Postgres => DbType::Postgres,
            crate::config::DbType::Sqlite => DbType::Sqlite,
            crate::config::DbType::Mysql => DbType::Mysql,
        }
    }
}

#[derive(Clone)]
pub struct DatabaseManager {
    #[cfg(feature = "postgres")]
    postgres_pool: Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
    organization_store: Arc<dyn OrganizationStore>,
    room_store: Arc<dyn RoomStore>,
    user_store: Arc<dyn UserStore>,
    message_store: Arc<dyn MessageStore>,
    event_store: Arc<dyn EventStore>,
    reaction_store: Arc<dyn ReactionStore>,
    db_type: DbType,
}

impl DatabaseManager {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let db_type = DbType::from(config.db_type);

        match db_type {
            #[cfg(feature = "postgres")]
            DbType::Postgres => {
                let connection_string = config.connection_string();
                let max_connections = config.max_connections().unwrap_or(10);
                let min_connections = config.min_connections().unwrap_or(1);

                let manager = ConnectionManager::<PgConnection>::new(connection_string);

                let pool = r2d2::Pool::builder()
                    .max_size(max_connections)
                    .min_idle(Some(min_connections))
                    .build(manager)
                    .map_err(|e| DatabaseError::Connection(e.to_string()))?;

                let organization_store = Arc::new(PostgresOrganizationStore::new(pool.clone()));
                let room_store = Arc::new(PostgresRoomStore::new(pool.clone()));
                let user_store = Arc::new(PostgresUserStore::new(pool.clone()));
                let message_store = Arc::new(PostgresMessageStore::new(pool.clone()));
                let event_store = Arc::new(PostgresEventStore::new(pool.clone()));
                let reaction_store = Arc::new(PostgresReactionStore::new(pool.clone()));

                Ok(Self {
                    postgres_pool: Some(pool),
                    organization_store,
                    room_store,
                    user_store,
                    message_store,
                    event_store,
                    reaction_store,
                    db_type,
                })
            }
            #[cfg(not(feature = "postgres"))]
            DbType::Postgres => {
                Err(DatabaseError::Connection("PostgreSQL support not compiled in".to_string()))
            }
            #[cfg(not(feature = "sqlite"))]
            DbType::Sqlite => {
                Err(DatabaseError::Connection("SQLite support not compiled in".to_string()))
            }
            #[cfg(not(feature = "mysql"))]
            DbType::Mysql => {
                Err(DatabaseError::Connection("MySQL support not compiled in".to_string()))
            }
            #[cfg(feature = "sqlite")]
            DbType::Sqlite => {
                todo!("SQLite support not yet implemented")
            }
            #[cfg(feature = "mysql")]
            DbType::Mysql => {
                todo!("MySQL support not yet implemented")
            }
        }
    }

    pub fn organization_store(&self) -> Arc<dyn OrganizationStore> {
        self.organization_store.clone()
    }

    pub fn room_store(&self) -> Arc<dyn RoomStore> {
        self.room_store.clone()
    }

    pub fn user_store(&self) -> Arc<dyn UserStore> {
        self.user_store.clone()
    }

    pub fn message_store(&self) -> Arc<dyn MessageStore> {
        self.message_store.clone()
    }

    pub fn event_store(&self) -> Arc<dyn EventStore> {
        self.event_store.clone()
    }

    pub fn reaction_store(&self) -> Arc<dyn ReactionStore> {
        self.reaction_store.clone()
    }

    pub fn db_type(&self) -> DbType {
        self.db_type
    }

    pub async fn migrate(&self) -> Result<()> {
        match self.db_type {
            #[cfg(feature = "postgres")]
            DbType::Postgres => {
                if let Some(pool) = &self.postgres_pool {
                    let mut conn = pool
                        .get()
                        .map_err(|e| DatabaseError::Connection(e.to_string()))?;
                    
                    tokio::task::spawn_blocking(move || {
                        diesel::sql_query(include_str!("../../migrations/postgres/001_init.sql"))
                            .execute(&mut conn)
                            .map_err(|e| DatabaseError::Migration(e.to_string()))
                    })
                    .await
                    .map_err(|e| DatabaseError::Migration(e.to_string()))??;
                }
                Ok(())
            }
            #[cfg(not(feature = "postgres"))]
            DbType::Postgres => {
                Err(DatabaseError::Migration("PostgreSQL support not compiled in".to_string()))
            }
            _ => Ok(()),
        }
    }
}
