#[derive(Debug)]
pub struct IssueInvoiceRequest {
    pub date: String,

    pub client_country: String,
    pub client_nif: i16,
    pub client_name: String,
    pub client_address: String,

    pub value: String,
    pub nif: String,
}

impl IssueInvoiceRequest {
    pub fn build_description(&self) -> String {
        "".to_owned()
    }
}
