use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IrsptApiAuth {
    async fn authenticate_async(&self, nif: &str, password: &str) -> Result<()>;
}
