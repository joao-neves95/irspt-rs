use crate::models::IssueInvoiceRequest;

use anyhow::Result;

pub trait InvoiceTemplateStore<'a> {
    fn get_template(&self, template_name: &str) -> Result<Option<IssueInvoiceRequest>>;

    fn add_template(&self, template_name: &str, model: &IssueInvoiceRequest) -> Result<()>;

    fn update_template(&self, template_name: &str, model: &IssueInvoiceRequest) -> Result<()>;

    fn remove_template(&self, template_name: &str) -> Result<()>;
}
