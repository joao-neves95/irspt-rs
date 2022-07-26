use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::{By, WebElement};

#[async_trait]
pub trait WebElementExtensions {
    async fn find_sub_elem_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement>;

    async fn select_option_by_prop_value_async(
        self,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<()>;
}

#[async_trait]
impl WebElementExtensions for WebElement {
    async fn find_sub_elem_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement> {
        Ok(self
            .find(By::Css(&format!(
                "{}[{}='{}']",
                elem_name, prop_name, prop_value
            )))
            .await?)
    }

    async fn select_option_by_prop_value_async(
        self,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<()> {
        self.find_sub_elem_by_prop_value_async("option", prop_name, prop_value)
            .await?
            .click()
            .await?;

        Ok(())
    }
}
