pub struct IssueInvoiceRequest {
    pub nif: String,
    // TODO: Format date to year-mm-dd
    pub date: String,
    pub activity: String,

    pub client_country: String,
    pub client_nif: i16,
    pub client_name: String,
    pub client_address: String,

    pub value: String,
    pub iva_regime: String,
}

impl IssueInvoiceRequest {
    pub fn build_description(&self) -> String {
        "".to_owned()
    }
}
