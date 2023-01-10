use crate::{infra::SledDb, models::IssueInvoiceRequest};

use anyhow::Result;

pub trait TInvoiceTemplateStore<'a> {
    fn new(db_wrapper: &'a mut SledDb) -> Result<Self>
    where
        Self: Sized;

    fn get_template(&self, template_name: &str) -> Result<Option<IssueInvoiceRequest>>;

    fn add_template(&self, template_name: &str, model: &IssueInvoiceRequest) -> Result<()>;

    fn update_template(&self, template_name: &str, model: &IssueInvoiceRequest) -> Result<()>;

    fn remove_template(&self, template_name: &str) -> Result<()>;
}
