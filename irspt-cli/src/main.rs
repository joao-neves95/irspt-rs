use anyhow::Result;
use irspt_api::{models::IssueInvoiceRequest, IrsptApi, IrsptApiAuth};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let irspt_api = &IrsptApi::new().await?;
    let auth_api = IrsptApiAuth::new(&irspt_api);
    auth_api
        .authenticate_async("142332425", "super_password123")
        .await?;

    let invoice_request = IssueInvoiceRequest {
        nif: "123".to_owned(),
        date: "2022-07-25".to_owned(),
    };

    Ok(())
}
