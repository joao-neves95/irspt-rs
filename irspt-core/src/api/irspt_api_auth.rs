use super::{extensions::WebDriverExtensions, IrsptApi};
use crate::traits::TIrsptApiAuth;

use std::{thread, time};

use anyhow::{Ok, Result};
use async_trait::async_trait;
use thirtyfour::By;

#[async_trait]
impl TIrsptApiAuth for IrsptApi {
    async fn authenticate_async(&self, nif: &str, password: &str) -> Result<&Self> {
        // TODO: Un-hardcode url.
        self.web_driver
            // .goto("https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/dashboard")
            .goto("https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/home")
            .await?;

        let _ = &self
            .web_driver
            .find_by_prop_value_async("label", "for", "tab2")
            .await?
            .click()
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_id_async("username", nif)
            .await?;

        let _ = &self
            .web_driver
            .set_input_value_by_id_async("password-nif", password)
            .await?;

        self.web_driver
            .find(By::Id("sbmtLogin"))
            .await?
            .click()
            .await?;

        thread::sleep(time::Duration::from_secs(2));

        Ok(self)
    }
}
