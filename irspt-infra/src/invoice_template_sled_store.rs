use anyhow::Result;
use async_trait::async_trait;

use irspt_core::{
    models::{IssueInvoiceRequest, Template},
    traits::{DbWrapper, InvoiceTemplateStore},
};

use crate::sled_db::SledDb;

pub struct InvoiceTemplateSledStore<'a> {
    sled_db: Box<&'a dyn DbWrapper<'a>>,
}

impl<'a> InvoiceTemplateSledStore<'a> {
    fn new(db_wrapper: &'a mut SledDb) -> Result<Self> {
        let _ = db_wrapper.open();

        Ok(InvoiceTemplateSledStore::<'a> {
            sled_db: Box::new(db_wrapper),
        })
    }
}

#[async_trait]
impl<'a> InvoiceTemplateStore<'a> for InvoiceTemplateSledStore<'a> {
    fn db_ref(&self) -> &Box<&'a dyn DbWrapper> {
        &self.sled_db
    }

    fn get_template(&self, template_name: &str) -> Result<Option<&Template<IssueInvoiceRequest>>> {
        todo!()
    }

    fn add_template(&self, model: &Template<IssueInvoiceRequest>) -> Result<()> {
        todo!()
    }

    fn update_template(&self, model: &Template<IssueInvoiceRequest>) -> Result<()> {
        todo!()
    }
}
