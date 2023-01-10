use super::{
    byte_serialization::{deserialize_from_bytes, serialize_to_bytes},
    SledDb,
};
use crate::{models::IssueInvoiceRequest, traits::TInvoiceTemplateStore};

use std::collections::HashMap;

use anyhow::{Ok, Result};

const INVOICE_TEAMPLATES_TABLE_NAME: &str = "INVOICE_TEAMPLATES";

pub struct InvoiceTemplateSledStore<'a> {
    sled_db: &'a SledDb,
}

impl<'a> TInvoiceTemplateStore<'a> for InvoiceTemplateSledStore<'a> {
    fn new(db_wrapper: &'a mut SledDb) -> Result<Self> {
        let _ = db_wrapper.open();

        Ok(InvoiceTemplateSledStore::<'a> {
            sled_db: db_wrapper,
        })
    }

    fn get_template(&self, template_name: &str) -> Result<Option<IssueInvoiceRequest>> {
        let raw_result = self
            .sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .get(template_name)?;

        if raw_result.is_none() {
            return Ok(None);
        }

        Ok(Some(IssueInvoiceRequest::new(deserialize_from_bytes::<
            HashMap<String, String>,
        >(
            &raw_result.unwrap()
        )?)))
    }

    fn add_template(&self, template_name: &str, model: &IssueInvoiceRequest) -> Result<()> {
        let model_bytes = serialize_to_bytes::<HashMap<String, String>>(&model.data)?;

        self.sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .insert(template_name, &model_bytes)?;

        Ok(())
    }

    fn update_template(&self, template_name: &str, model: &IssueInvoiceRequest) -> Result<()> {
        self.add_template(template_name, model)?;

        Ok(())
    }

    fn remove_template(&self, template_name: &str) -> Result<()> {
        self.sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .remove(template_name)?;

        Ok(())
    }
}
