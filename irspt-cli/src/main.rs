mod prompt;
mod validators;
use irspt_contracts::{models::IssueInvoiceRequest, traits::InvoiceTemplateStore};
use irspt_infra::{InvoiceTemplateSledStore, SledDb};

use std::{thread, time};

use anyhow::{Context, Result};
use inquire::Confirm;
use prompt::prompt_invoice_request;

const DEFAULT_TEMPLATE_NAME: &str = "DEFAULT";

#[tokio::main]
async fn main() -> Result<()> {
    let mut sled_db = SledDb::new();
    let invoice_template_store = InvoiceTemplateSledStore::new(&mut sled_db)?;

    let existing_model = match invoice_template_store.get_template(DEFAULT_TEMPLATE_NAME) {
        anyhow::Result::Ok(model) => model,

        Err(_) => delete_template_if_invalid_prompt(&invoice_template_store)?,
    };

    let invoice_request = prompt_invoice_request(&existing_model)?;

    let save_template = Confirm::new("Save as template?")
        .with_default(false)
        .with_help_message("Your password will not be stored.")
        .prompt()?;

    if save_template {
        invoice_template_store.update_template(DEFAULT_TEMPLATE_NAME, &invoice_request)?;
    }

    #[cfg(feature = "issue-invoice")]
    {
        use inquire::{required, Password};
        use irspt_api::IrsptApi;
        use irspt_contracts::traits::{IrsptApiAuth, IrsptApiInvoices};

        let password = Password::new("Password:")
            .with_validator(required!())
            .prompt()?;

        let irspt_api = IrsptApi::new().await.context(
            "ERROR: Issue while trying to connect to the WebDriver server. Make sure it's running.",
        )?;

        irspt_api
            .authenticate_async(&invoice_request.get_nif(), &password)
            .await?;

        irspt_api.issue_invoice_async(&invoice_request).await?;

        thread::sleep(time::Duration::from_secs(5));
        irspt_api.close_async().await?;
    }

    Ok(())
}

fn delete_template_if_invalid_prompt<'a>(
    invoice_template_store: &impl InvoiceTemplateStore<'a>,
) -> Result<Option<IssueInvoiceRequest>> {
    let delete_template = Confirm::new(
        "ERROR: Your template data was corrupted. Delete the existing one to create a new one?",
    )
    .with_default(true)
    .prompt()?;

    if !delete_template {
        return Ok(None);
    }

    invoice_template_store.remove_template(&DEFAULT_TEMPLATE_NAME)?;

    Ok(None)
}
