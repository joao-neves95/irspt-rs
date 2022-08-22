use crate::models::IssueInvoiceRequest;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IrsptApiInvoices {
    async fn issue_invoice_async(&self, request_model: &IssueInvoiceRequest) -> Result<()>;

    // async fn get_reference_data() -> Result<()>;
}
