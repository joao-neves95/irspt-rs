use crate::{
    api::{IrsptApi, IrsptApiProps},
    enums::{InstanceState, WebdriverType},
    infra::{InvoiceTemplateSledStore, WebdriverManager},
    models::IssueInvoiceRequest,
    traits::{
        TInvoiceTemplateStore, TInvoicesService, TIrsptApiAuth, TIrsptApiInvoices,
        TWebdriverManager,
    },
};

use anyhow::{Context, Result};
use async_trait::async_trait;

const DEFAULT_TEMPLATE_NAME: &str = "DEFAULT";

pub struct InvoicesServiceProps<'a> {
    pub headless_webdriver: bool,
    pub invoice_template_store: &'a InvoiceTemplateSledStore<'a>,
}

pub struct InvoicesService<'a> {
    invoice_template_store: Box<&'a InvoiceTemplateSledStore<'a>>,
    webdriver_instance_state: InstanceState,
    irspt_api: IrsptApi,
}

#[async_trait]
impl<'a> TInvoicesService<'a> for InvoicesService<'a> {
    async fn new_async(props: &'a InvoicesServiceProps) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(InvoicesService {
            invoice_template_store: Box::new(props.invoice_template_store),

            webdriver_instance_state: WebdriverManager::new(WebdriverType::Gecko).start_instance_if_needed()?,

            irspt_api: IrsptApi::new( IrsptApiProps { headless: props.headless_webdriver }).await.context(
                "ERROR: Issue while trying to connect to the WebDriver server. Make sure it's running.",
            )?,
        })
    }

    async fn drop_async(&mut self) -> Result<()> {
        match &mut self.webdriver_instance_state {
            InstanceState::Started(process) => {
                let _ = process.kill();
            }
            _ => (), // Nothing to do.
        }

        Ok(())
    }

    fn get_saved_template(&self) -> Result<Option<IssueInvoiceRequest>> {
        self.invoice_template_store
            .get_template(DEFAULT_TEMPLATE_NAME)
    }

    fn update_saved_template(&self, invoice_request: &IssueInvoiceRequest) -> Result<()> {
        self.invoice_template_store
            .update_template(DEFAULT_TEMPLATE_NAME, &invoice_request)
    }

    fn delete_saved_template(&self) -> Result<()> {
        self.invoice_template_store
            .remove_template(&DEFAULT_TEMPLATE_NAME)
    }

    async fn authenticate_async(&self, nif: &String, password: &String) -> Result<()> {
        self.irspt_api.authenticate_async(&nif, &password).await?;

        Ok(())
    }

    async fn issue_invoice_async(&self, invoice_request: &IssueInvoiceRequest) -> Result<()> {
        self.irspt_api.issue_invoice_async(&invoice_request).await?;

        Ok(())
    }
}
