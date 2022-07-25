use anyhow::Result;
use thirtyfour::{By, WebDriver, WebElement};

pub struct WebDriverActions {}

impl WebDriverActions {
    pub async fn find_elem_by_prop_value(
        driver: &WebDriver,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement> {
        Ok(driver
            .find(By::Css(
                format!("{}[{}='{}']", elem_name, prop_name, prop_value).as_str(),
            ))
            .await?)
    }

    pub async fn find_sub_elem_by_prop_value(
        select_elem: &WebElement,
        elem_name: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<WebElement> {
        Ok(select_elem
            .find(By::Css(
                format!("{}[{}='{}']", elem_name, prop_name, prop_value).as_str(),
            ))
            .await?)
    }

    pub async fn set_input_value_by_id(driver: &WebDriver, id: &str, value: &str) -> Result<()> {
        driver.find(By::Id(id)).await?.send_keys(value).await?;

        Ok(())
    }

    pub async fn set_input_value_by_prop_value(
        driver: &WebDriver,
        prop_name: &str,
        prop_value: &str,
        value: &str,
    ) -> Result<()> {
        WebDriverActions::find_elem_by_prop_value(driver, "input", prop_name, prop_value)
            .await?
            .send_keys(value)
            .await?;

        Ok(())
    }

    pub async fn select_option_prop_by_prop_value(
        select_elem: &WebElement,
        prop_name: &str,
        prop_value: &str,
    ) -> Result<()> {
        WebDriverActions::find_sub_elem_by_prop_value(select_elem, "option", prop_name, prop_value)
            .await?
            .click()
            .await?;

        Ok(())
    }
}
