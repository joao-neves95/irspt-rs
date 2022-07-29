mod validators;
use irspt_core::models::IssueInvoiceRequest;

use anyhow::Result;
use inquire::{required, DateSelect, Password, Text};

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Un-hardcode the prompt messages.
    // TODO: Extract the prompt into a IssueInvoiceRequest prompt builder.
    let invoice_request = IssueInvoiceRequest {
        date: DateSelect::new("Service date:")
            .with_week_start(chrono::Weekday::Mon)
            .with_default(chrono::Local::today().naive_local())
            .prompt()?
            .format("%Y-%m-%d")
            .to_string(),

        client_country: Text::new("Client Country:")
            .with_help_message("(e.g. PORTUGAL)")
            .with_validator(required!())
            .prompt()?
            .to_string()
            .to_uppercase(),

        client_nif: Text::new("Client NIF/VAT:")
            .with_validators(&[required!(), is_integer!()])
            .prompt()?
            .to_string(),

        client_name: Text::new("Client Name:")
            .with_validator(required!())
            .prompt()?
            .to_string(),

        client_address: Text::new("Client Address:")
            .with_validator(required!())
            .prompt()?
            .to_string(),

        value: Text::new("Value (€):")
            .with_validators(&[required!(), is_decimal!()])
            .prompt()?
            .to_string(),

        nif: Text::new("NIF:")
            .with_validators(&[required!(), is_integer!()])
            .prompt()?
            .to_string(),
    };

    let password = Password::new("Password:")
        .with_validator(required!())
        .prompt()?;

    #[cfg(feature = "issue-invoice")]
    {
        use irspt_api::{IrsptApi, IrsptApiAuth};

        let irspt_api = IrsptApi::new().await?;
        let auth_api = IrsptApiAuth::new(&irspt_api);
        auth_api
            .authenticate_async(&invoice_request.nif, &password)
            .await?;
    }

    Ok(())
}
