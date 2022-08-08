use crate::{
    byte_serialization::{deserialize_from_bytes, serialize_to_bytes},
    sled_db::SledDb,
};
use irspt_contracts::{models::InvoiceTemplate, traits::InvoiceTemplateStore};

use anyhow::{Ok, Result};

const INVOICE_TEAMPLATES_TABLE_NAME: &str = "INVOICE_TEAMPLATES";
const INVOICE_TEAMPLATES_VERSION_TABLE_NAME: &str = "INVOICE_TEAMPLATES_V";

pub struct InvoiceTemplateSledStore<'a> {
    sled_db: &'a SledDb,
}

impl<'a> InvoiceTemplateSledStore<'a> {
    pub fn new(db_wrapper: &'a mut SledDb) -> Result<Self> {
        let _ = db_wrapper.open();

        Ok(InvoiceTemplateSledStore::<'a> {
            sled_db: db_wrapper,
        })
    }
}

impl<'a> InvoiceTemplateStore<'a> for InvoiceTemplateSledStore<'a> {
    fn get_saved_template_version(&self) -> Result<Option<u8>> {
        let raw_result = self
            .sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .get(INVOICE_TEAMPLATES_VERSION_TABLE_NAME)?;

        if raw_result.is_none() {
            return Ok(None);
        }

        Ok(Some(deserialize_from_bytes::<u8>(&raw_result.unwrap())?))
    }

    fn set_saved_template_version(&self, version: u8) -> Result<()> {
        self.sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .insert(
                INVOICE_TEAMPLATES_VERSION_TABLE_NAME,
                serialize_to_bytes(&version)?,
            )?;

        Ok(())
    }

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

        Ok(Some(deserialize_from_bytes::<InvoiceTemplate>(
            &raw_result.unwrap(),
        )?))
    }

    fn add_template(&self, model: &InvoiceTemplate) -> Result<()> {
        let model_bytes = serialize_to_bytes(model)?;

        self.sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .insert(&model.name, model_bytes)?;

        Ok(())
    }

    fn update_template(&self, model: &InvoiceTemplate) -> Result<()> {
        self.add_template(model)?;

        Ok(())
    }

    fn remove_template(&self, template_name: &str) -> Result<()> {
        self.sled_db
            .db_ref()
            .unwrap()
            .open_tree(INVOICE_TEAMPLATES_TABLE_NAME)?
            .remove(&template_name)?;

        Ok(())
    }
}
