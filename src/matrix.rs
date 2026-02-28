pub struct MatrixAppservice;

impl MatrixAppservice {
    pub fn new() -> crate::utils::Result<Self> {
        Ok(Self)
    }

    pub async fn start(&self) -> crate::utils::Result<()> {
        Ok(())
    }
}
