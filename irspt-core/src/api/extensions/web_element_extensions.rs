use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::{By, WebElement};

#[async_trait]
pub trait WebElementExtensions {
    async fn find_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement>;

    async fn find_by_prop_value_containing_async(
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

    async fn select_option_by_prop_value_containing_async(
        self,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<()>;
}

fn find_by_prop_selector(
    elem_name: &str,
    prop_name: &str,
    prop_value: &str,
    like_query: bool,
) -> By {
    By::Css(&format!(
        "{}[{}{}='{}']",
        elem_name,
        prop_name,
        if like_query { "*" } else { "" },
        prop_value
    ))
}

#[async_trait]
impl WebElementExtensions for WebElement {
    async fn find_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement> {
        Ok(self
            .find(find_by_prop_selector(
                elem_name, prop_name, prop_value, false,
            ))
            .await?)
    }

    async fn find_by_prop_value_containing_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement> {
        Ok(self
            .find(find_by_prop_selector(
                elem_name, prop_name, prop_value, true,
            ))
            .await?)
    }

    async fn select_option_by_prop_value_async(
        self,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<()> {
        self.find_by_prop_value_async("option", prop_name, prop_value)
            .await?
            .click()
            .await?;

        Ok(())
    }

    async fn select_option_by_prop_value_containing_async(
        self,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<()> {
        self.find_by_prop_value_containing_async("option", prop_name, prop_value)
            .await?
            .click()
            .await?;

        Ok(())
    }
}
