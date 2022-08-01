mod prompt;
mod validators;
use irspt_core::{models::InvoiceTemplate, traits::InvoiceTemplateStore};
use irspt_infra::{InvoiceTemplateSledStore, SledDb};

use anyhow::Result;
use inquire::Confirm;
use prompt::prompt_invoice_request;

const DEFAULT_TEMPLATE_NAME: &str = "DEFAULT";

#[tokio::main]
async fn main() -> Result<()> {
    let mut sled_db = SledDb::new();
    let invoice_template_store = Some(InvoiceTemplateSledStore::new(&mut sled_db)?);

    let existing_model = match invoice_template_store
        .as_ref()
        .unwrap()
        .get_template(DEFAULT_TEMPLATE_NAME)?
    {
        None => None,
        Some(template) => Some(template.invoice_model),
    };

    let invoice_request = prompt_invoice_request(&existing_model)?;

    let save_template = Confirm::new("Save as template?")
        .with_default(false)
        .with_help_message("Your password will not be stored.")
        .prompt()?;

    let _nif = invoice_request.nif.to_owned();

    if save_template {
        let template = InvoiceTemplate {
            name: DEFAULT_TEMPLATE_NAME.to_owned(),
            invoice_model: invoice_request,
        };

        if existing_model.is_some() {
            invoice_template_store
                .as_ref()
                .unwrap()
                .update_template(&template)?;
        } else {
            invoice_template_store
                .as_ref()
                .unwrap()
                .add_template(&template)?;
        }
    }

    #[cfg(feature = "issue-invoice")]
    {
        use inquire::{required, Password};
        use irspt_api::{IrsptApi, IrsptApiAuth};

        let password = Password::new("Password:")
            .with_validator(required!())
            .prompt()?;

        let irspt_api = IrsptApi::new().await?;
        let auth_api = IrsptApiAuth::new(&irspt_api);
        auth_api.authenticate_async(&_nif, &password).await?;

        irspt_api.close_async().await?;
    }

    Ok(())
}
