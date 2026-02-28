use async_trait::async_trait;

use crate::db::error::Result;
use crate::db::models::{Organization, OrganizationChangeset};

#[async_trait]
pub trait OrganizationStore: Send + Sync {
    async fn create(&self, org: Organization) -> Result<Organization>;
    
    async fn get(&self, id: &str) -> Result<Option<Organization>>;
    
    async fn get_all(&self) -> Result<Vec<Organization>>;
    
    async fn update(&self, id: &str, changeset: OrganizationChangeset) -> Result<Organization>;
    
    async fn delete(&self, id: &str) -> Result<()>;
    
    async fn set_connected(&self, id: &str, connected: bool) -> Result<()>;
    
    async fn exists(&self, id: &str) -> Result<bool>;
}
