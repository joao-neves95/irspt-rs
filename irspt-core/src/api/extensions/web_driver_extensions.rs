use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::{By, WebDriver, WebElement};

use crate::api::constants::HtmlTagNames;

pub struct ElementProp<'a> {
    pub prop_name: &'a str,
    pub prop_value: &'a str,
}

#[async_trait]
pub trait WebDriverExtensions {
    async fn find_by_prop_value_async(
        &self,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement>;

    async fn find_by_props_value_async(
        &self,
        elem_name: &str,
        props: &[&ElementProp],
    ) -> Result<WebElement>;

    async fn set_input_value_by_id_async(&self, id: &str, value: &str) -> Result<()>;

    async fn set_input_value_by_prop_value_async(
        &self,
        prop_name: &str,
        prop_value: &str,
        value: &str,
    ) -> Result<()>;

    async fn set_textarea_value_by_prop_value_async(
        &self,
        prop_name: &str,
        prop_value: &str,
        value: &str,
    ) -> Result<()>;
}

#[async_trait]
impl WebDriverExtensions for WebDriver {
    async fn find_by_prop_value_async(
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

    async fn find_by_props_value_async(
        &self,
        elem_name: &str,
        props: &[&ElementProp],
    ) -> Result<WebElement> {
        let props_selector = props
            .iter()
            .map(|prop| format!("[{}='{}']", prop.prop_name, prop.prop_value))
            .collect::<String>();

        Ok(self
            .find(By::Css(&format!("{}{}", elem_name, props_selector)))
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
        self.find_by_prop_value_async(HtmlTagNames::INPUT, prop_name, prop_value)
            .await?
            .send_keys(value)
            .await?;

        Ok(())
    }

    async fn set_textarea_value_by_prop_value_async(
        &self,
        prop_name: &str,
        prop_value: &str,
        value: &str,
    ) -> Result<()> {
        self.find_by_prop_value_async(HtmlTagNames::TEXTAREA, prop_name, prop_value)
            .await?
            .send_keys(value)
            .await?;

        Ok(())
    }
}
