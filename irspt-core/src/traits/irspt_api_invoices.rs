use crate::models::IssueInvoiceRequest;
#[cfg(feature = "reference-data")]
use crate::models::ReferenceDataDto;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TIrsptApiInvoices<'a> {
    async fn issue_invoice_async(&self, request_model: &IssueInvoiceRequest) -> Result<()>;

    #[cfg(feature = "reference-data")]
    async fn get_reference_data(
        &self,
        dto_ref: &'a mut ReferenceDataDto,
    ) -> Result<&'a ReferenceDataDto>;
}
