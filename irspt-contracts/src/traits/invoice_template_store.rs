use crate::models::InvoiceTemplate;

use anyhow::Result;

pub trait InvoiceTemplateStore<'a> {
    fn get_saved_template_version(&self) -> Result<Option<u8>>;

    fn set_saved_template_version(&self, version: u8) -> Result<()>;

    fn get_template(&self, template_name: &str) -> Result<Option<InvoiceTemplate>>;

    fn add_template(&self, model: &InvoiceTemplate) -> Result<()>;

    fn update_template(&self, model: &InvoiceTemplate) -> Result<Option<u8>>;

    fn remove_template(&self, template_name: &str) -> Result<()>;
}
