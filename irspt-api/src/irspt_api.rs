use anyhow::{Error, Result};
use thirtyfour::{DesiredCapabilities, WebDriver};

pub struct IrsptApi {
    pub web_driver: WebDriver,
}

impl IrsptApi {
    pub async fn new() -> Result<IrsptApi, Error> {
        let caps = DesiredCapabilities::firefox();
        // TODO: Un-hardcode url.
        let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

        Ok(IrsptApi { web_driver: driver })
    }

    pub async fn close_async(self) -> Result<()> {
        self.web_driver.quit().await?;

        Ok(())
    }
}
