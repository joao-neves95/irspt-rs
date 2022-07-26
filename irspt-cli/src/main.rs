use anyhow::Result;
use inquire::DateSelect;

use irspt_api::{models::IssueInvoiceRequest, IrsptApi, IrsptApiAuth};

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Un-hardcode string messages.
    // TODO: Extract the prompt into a IssueInvoiceRequest prompt builder.
    let invoice_request = IssueInvoiceRequest {
        date: DateSelect::new("Service date")
            .with_week_start(chrono::Weekday::Mon)
            .with_default(chrono::Local::today().naive_local())
            .prompt()?
            .format("%Y-%m-%d")
            .to_string(),

        nif: "".to_owned(),

        client_country: "".to_owned(),
        client_nif: 12345,
        client_name: "".to_owned(),
        client_address: "".to_owned(),

        value: "".to_owned(),
    };

    let password = "".to_owned();

    let irspt_api = IrsptApi::new().await?;
    let auth_api = IrsptApiAuth::new(&irspt_api);
    auth_api
        .authenticate_async(&invoice_request.nif, &password)
        .await?;

    Ok(())
}
