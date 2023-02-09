use anyhow::{Error, Result};
use thirtyfour::{DesiredCapabilities, WebDriver};

pub struct IrsptApiProps {
    pub headless: bool,
}

pub struct IrsptApi {
    pub(crate) web_driver: WebDriver,
}

impl IrsptApi {
    pub async fn new(props: IrsptApiProps) -> Result<IrsptApi, Error> {
        let mut caps = DesiredCapabilities::firefox();

        if props.headless {
            caps.set_headless()?;
        }

        // TODO: Un-hardcode url.
        let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

        Ok(IrsptApi { web_driver: driver })
    }

    pub async fn close_async(self) -> Result<()> {
        self.web_driver.quit().await?;

        Ok(())
    }
}
