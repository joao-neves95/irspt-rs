use anyhow::Result;
use irspt_api::{IrsptApi, IrsptApiAuth};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let irspt_api = &IrsptApi::new().await?;
    let auth_api = IrsptApiAuth::new(&irspt_api);
    let cookie = auth_api.authenticate_async("142332425", "super_password123");

    Ok(())
}
