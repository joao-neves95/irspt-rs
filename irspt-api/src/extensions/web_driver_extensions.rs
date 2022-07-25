use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::{By, WebDriver, WebElement};

#[async_trait]
pub trait WebDriverExtensions {
    async fn find_elem_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement>;

    async fn set_input_value_by_id_async(&self, id: &str, value: &str) -> Result<()>;

    async fn set_input_value_by_prop_value_async(
        &self,
        prop_name: &str,
        prop_value: &str,
        value: &str,
    ) -> Result<()>;
}

#[async_trait]
impl WebDriverExtensions for WebDriver {
    async fn find_elem_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement> {
        Ok(self
            .find(By::Css(
                format!("{}[{}='{}']", elem_name, prop_name, prop_value).as_str(),
            ))
            .await?)
    }

    async fn set_input_value_by_id_async(&self, id: &str, value: &str) -> Result<()> {
        self.find(By::Id(id)).await?.send_keys(value).await?;

        Ok(())
    }

    async fn set_input_value_by_prop_value_async(
        &self,
        prop_name: &str,
        prop_value: &str,
        value: &str,
    ) -> Result<()> {
        self.find_elem_by_prop_value_async("input", prop_name, prop_value)
            .await?
            .send_keys(value)
            .await?;

        Ok(())
    }
}
