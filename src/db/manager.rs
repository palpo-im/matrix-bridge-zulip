pub struct DatabaseManager;

impl DatabaseManager {
    pub fn new(_url: &str) -> crate::utils::Result<Self> {
        Ok(Self)
    }

    pub async fn migrate(&self) -> crate::utils::Result<()> {
        Ok(())
    }
}
