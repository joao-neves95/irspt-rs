mod prompt;
mod validators;

use irspt_core::{
    infra::{InvoiceTemplateSledStore, SledDb},
    models::IssueInvoiceRequest,
    services::{InvoicesService, InvoicesServiceProps},
    traits::{TInvoiceTemplateStore, TInvoicesService},
};
use prompt::prompt_invoice_request;

use anyhow::Result;
use inquire::{required, Confirm, Password, Text};

#[tokio::main]
async fn main() -> Result<()> {
    let mut sled_db = SledDb::new();
    let _ = sled_db.open();

    let invoice_template_store = InvoiceTemplateSledStore::new(&sled_db)?;

    let invoices_service_props = InvoicesServiceProps {
        invoice_template_store: &invoice_template_store,
        headless_webdriver: cfg!(feature = "headless-webdriver"),
    };

    let mut invoice_service = InvoicesService::new_async(&invoices_service_props).await?;

    let existing_model = match invoice_service.get_saved_template() {
        anyhow::Result::Ok(model) => model,

        Err(_) => delete_template_if_invalid_prompt(&invoice_service)?,
    };

    let nif = Text::new("NIF:")
        .with_validators(&[required!(), is_integer!()])
        .with_default(if existing_model.is_some() {
            existing_model.as_ref().unwrap().get_nif()
        } else {
            ""
        })
        .prompt()?
        .to_string();

    let password = Password::new("Password:")
        .with_validator(required!())
        .prompt()?;

    invoice_service.authenticate_async(&nif, &password).await?;

    let mut invoice_request = prompt_invoice_request(&existing_model)?;
    invoice_request.set_nif(nif);

    let save_template = Confirm::new("Save as template?")
        .with_default(false)
        .with_help_message("Your password will not be stored.")
        .prompt()?;

    if save_template {
        invoice_service.update_saved_template(&invoice_request)?;
    }

    #[cfg(feature = "issue-invoice")]
    {
        invoice_service
            .issue_invoice_async(&invoice_request)
            .await?;

        #[cfg(feature = "issue-invoice-final-timout")]
        {
            use core::time;
            use std::thread;

            thread::sleep(time::Duration::from_secs(5));
        }
    }

    invoice_service.drop_async().await?;

    Ok(())
}

fn delete_template_if_invalid_prompt<'a>(
    invoice_service: &impl TInvoicesService<'a>,
) -> Result<Option<IssueInvoiceRequest>> {
    let delete_template = Confirm::new(
        "ERROR: Your template data was corrupted. Delete the existing one to create a new one?",
    )
    .with_default(true)
    .prompt()?;

    if !delete_template {
        return Ok(None);
    }

    invoice_service.delete_saved_template()?;

    Ok(None)
}
