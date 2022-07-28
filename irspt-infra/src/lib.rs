mod sled_db;
use sled_db::SledDb;

mod invoice_template_sled_store;
pub use invoice_template_sled_store::InvoiceTemplateSledStore;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
