use anyhow::Result;
use async_trait::async_trait;

use crate::models::{IssueInvoiceRequest, Template};

#[async_trait]
pub trait InvoiceTemplateStore {
    fn get_template(template_name: &str) -> Result<&Template<IssueInvoiceRequest>>;

    fn add_template(model: &Template<IssueInvoiceRequest>) -> Result<()>;

    fn update_template(model: &Template<IssueInvoiceRequest>) -> Result<()>;
}
