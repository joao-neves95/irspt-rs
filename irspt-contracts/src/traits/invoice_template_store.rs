use crate::models::InvoiceTemplate;

use anyhow::Result;

pub trait InvoiceTemplateStore<'a> {
    fn get_template(&self, template_name: &str) -> Result<Option<InvoiceTemplate>>;

    fn add_template(&self, model: &InvoiceTemplate) -> Result<()>;

    fn update_template(&self, model: &InvoiceTemplate) -> Result<Option<InvoiceTemplate>>;

    fn remove_template(&self, template_name: &str) -> Result<()>;
}
