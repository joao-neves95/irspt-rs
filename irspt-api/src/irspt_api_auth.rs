use anyhow::{Ok, Result};
use thirtyfour::By;

use crate::{extensions::WebDriverExtensions, IrsptApi};

pub struct IrsptApiAuth<'a> {
    irspt_api: &'a IrsptApi,
}

impl<'a> IrsptApiAuth<'a> {
    pub fn new(irspt_api: &'a IrsptApi) -> IrsptApiAuth {
        IrsptApiAuth { irspt_api }
    }

    pub async fn authenticate_async(&self, nif: &str, password: &str) -> Result<()> {
        // TODO: Un-hardcode url.
        self.irspt_api
            .web_driver
            // .goto("https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/dashboard")
            .goto("https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/home")
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_id_async("username", nif)
            .await?;

        let _ = &self
            .irspt_api
            .web_driver
            .set_input_value_by_id_async("password-nif", password)
            .await?;

        self.irspt_api
            .web_driver
            .find(By::Id("sbmtLogin"))
            .await?
            .click()
            .await?;

        Ok(())
    }
}
