use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DbWrapper<'a> {
    fn open(&mut self) -> Result<()>;
}
