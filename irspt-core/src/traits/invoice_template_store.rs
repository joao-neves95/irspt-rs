use anyhow::Result;
use async_trait::async_trait;

use crate::models::{IssueInvoiceRequest, Template};
use crate::traits::DbWrapper;

#[async_trait]
pub trait InvoiceTemplateStore<'a> {
    fn db_ref(&self) -> &Box<&'a dyn DbWrapper>;

    fn get_template(&self, template_name: &str) -> Result<Option<&Template<IssueInvoiceRequest>>>;

    fn add_template(&self, model: &Template<IssueInvoiceRequest>) -> Result<()>;

    fn update_template(&self, model: &Template<IssueInvoiceRequest>) -> Result<()>;
}
