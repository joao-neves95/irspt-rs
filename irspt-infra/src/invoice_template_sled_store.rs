use crate::sled_db::SledDb;
use irspt_core::{models::InvoiceTemplate, traits::InvoiceTemplateStore};

use anyhow::{Ok, Result};
use async_trait::async_trait;

const INVOICE_TEAMPLATES_TABLE_NAME: &str = "INVOICE_TEAMPLATES";

pub struct InvoiceTemplateSledStore<'a> {
    sled_db: Box<&'a SledDb>,
}

impl<'a> InvoiceTemplateSledStore<'a> {
    pub fn new(db_wrapper: &'a mut SledDb) -> Result<Self> {
        let _ = db_wrapper.open();

        Ok(InvoiceTemplateSledStore::<'a> {
            sled_db: Box::new(db_wrapper),
        })
    }
}

#[async_trait]
impl<'a> InvoiceTemplateStore<'a> for InvoiceTemplateSledStore<'a> {
    fn get_template(&self, template_name: &str) -> Result<Option<InvoiceTemplate>> {
        let raw_result = self
            .sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .get(template_name)?;

        if raw_result.is_none() {
            return Ok(None);
        }

        let raw_bytes = sled::IVec::from(raw_result.unwrap());
        let template = rkyv::from_bytes::<InvoiceTemplate>(&raw_bytes).unwrap();
        Ok(Some(template))
    }

    fn add_template(&self, model: &InvoiceTemplate) -> Result<()> {
        todo!()
    }

    fn update_template(&self, model: &InvoiceTemplate) -> Result<()> {
        todo!()
    }
}
