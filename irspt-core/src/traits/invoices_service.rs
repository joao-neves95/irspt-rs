use crate::{models::IssueInvoiceRequest, services::InvoicesServiceProps};

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TInvoicesService<'a> {
    async fn new_async(props: &'a InvoicesServiceProps) -> Result<Self>
    where
        Self: Sized;

    async fn drop_async(&mut self) -> Result<()>;

    fn get_saved_template(&self) -> Result<Option<IssueInvoiceRequest>>;

    fn update_saved_template(&self, invoice_request: &IssueInvoiceRequest) -> Result<()>;

    fn delete_saved_template(&self) -> Result<()>;

    async fn authenticate_async(&self, nif: &String, password: &String) -> Result<()>;

    async fn issue_invoice_async(&self, invoice_request: &IssueInvoiceRequest) -> Result<()>;
}
