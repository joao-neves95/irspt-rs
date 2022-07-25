use anyhow::{Ok, Result};
use thirtyfour::By;

use crate::IrsptApi;

pub struct IrsptApiAuth<'a> {
    irspt_api: &'a IrsptApi,
}

impl<'a> IrsptApiAuth<'a> {
    pub fn new(irspt_api: &'a IrsptApi) -> IrsptApiAuth {
        IrsptApiAuth { irspt_api }
    }

    pub async fn authenticate_async(&self, username: &str, password: &str) -> Result<()> {
        // TODO: Un-hardcode url.
        self.irspt_api
            .web_driver
            .goto("https://www.acesso.gov.pt/v2/loginForm?partID=PFAP&path=/geral/dashboard")
            .await?;

        let username_elem = self.irspt_api.web_driver.find(By::Id("username")).await?;
        username_elem.send_keys(username).await?;

        let password_elem = self
            .irspt_api
            .web_driver
            .find(By::Id("password-nif"))
            .await?;
        password_elem.send_keys(password).await?;

        let submit_elem = self.irspt_api.web_driver.find(By::Id("sbmtLogin")).await?;
        submit_elem.click().await?;

        Ok(())
    }
}
