use super::IssueInvoiceRequest;

use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct InvoiceTemplate {
    pub name: String,
    pub invoice_model: IssueInvoiceRequest,
}
