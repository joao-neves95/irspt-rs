use std::collections::HashMap;

pub struct IssueInvoiceRequest {
    pub is_dev_mode: bool,
    pub data: HashMap<String, String>,
    pub(crate) default_string_value: String,
}

impl IssueInvoiceRequest {
    pub fn new(data: HashMap<String, String>) -> Self {
        IssueInvoiceRequest {
            is_dev_mode: false,
            data,
            default_string_value: String::new(),
        }
    }

    pub fn new_empty() -> Self {
        IssueInvoiceRequest {
            is_dev_mode: false,
            data: HashMap::new(),
            default_string_value: String::new(),
        }
    }

    pub fn set_is_dev_mode(&mut self, is: bool) -> &mut Self {
        self.is_dev_mode = is;

        self
    }

    pub fn get_date(&self) -> &String {
        get_model_data_value(self, "date")
    }

    pub fn set_date(&mut self, value: String) -> &Self {
        set_model_data_value(self, "date", value);

        self
    }

    pub fn get_description(&self) -> &String {
        get_model_data_value(self, "description")
    }

    pub fn set_description(&mut self, value: String) -> &Self {
        set_model_data_value(self, "description", value);

        self
    }

    pub fn get_client_country(&self) -> &String {
        get_model_data_value(&self, "client_country")
    }

    pub fn set_client_country(&mut self, value: String) -> &Self {
        set_model_data_value(self, "client_country", value);

        self
    }

    pub fn get_client_nif(&self) -> &String {
        get_model_data_value(self, "client_nif")
    }

    pub fn set_client_nif(&mut self, value: String) -> &Self {
        set_model_data_value(self, "client_nif", value);

        self
    }

    pub fn get_client_name(&self) -> &String {
        get_model_data_value(self, "client_name")
    }

    pub fn set_client_name(&mut self, value: String) -> &Self {
        set_model_data_value(self, "client_name", value);

        self
    }

    pub fn get_client_address(&self) -> &String {
        get_model_data_value(self, "client_address")
    }

    pub fn set_client_address(&mut self, value: String) -> &Self {
        set_model_data_value(self, "client_address", value);

        self
    }

    pub fn get_value(&self) -> &String {
        get_model_data_value(self, "value")
    }

    pub fn set_value(&mut self, value: String) -> &Self {
        set_model_data_value(self, "value", value);

        self
    }

    pub fn get_nif(&self) -> &String {
        get_model_data_value(self, "nif")
    }

    pub fn set_nif(&mut self, value: String) -> &Self {
        set_model_data_value(self, "nif", value);

        self
    }
}

fn get_model_data_value<'a>(model: &'a IssueInvoiceRequest, key: &str) -> &'a String {
    model.data.get(key).unwrap_or(&model.default_string_value)
}

fn set_model_data_value<'a>(
    model: &'a mut IssueInvoiceRequest,
    key: &str,
    new_value: String,
) -> () {
    model.data.insert(key.to_owned(), new_value);
}

impl From<&HashMap<String, String>> for IssueInvoiceRequest {
    fn from(hash_map: &HashMap<String, String>) -> Self {
        IssueInvoiceRequest::new(hash_map.to_owned())
    }
}

impl Into<HashMap<String, String>> for &IssueInvoiceRequest {
    fn into(self) -> HashMap<String, String> {
        self.data.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        issue_invoice_request::{get_model_data_value, set_model_data_value},
        IssueInvoiceRequest,
    };

    use std::collections::HashMap;

    #[test]
    fn get_hashmap_column_passes() -> () {
        let model = IssueInvoiceRequest::new(HashMap::from(get_small_test_table()));

        let result = get_model_data_value(&model, "test_column");
        assert_eq!(result, "test_value");
    }

    #[test]
    fn set_hashmap_column_passes() -> () {
        let mut model = IssueInvoiceRequest::new(HashMap::from(get_small_test_table()));
        set_model_data_value(&mut model, "test_column", "new_test_value".to_owned());

        assert_eq!(model.data.get("test_column").unwrap(), "new_test_value");
    }

    #[test]
    fn to_hashmap_passes() -> () {
        let test_data = IssueInvoiceRequest::new(HashMap::from(get_test_table()));

        let table: HashMap<String, String> = (&test_data).into();
        assert_eq!(table.get_key_value("date").unwrap().1, &"2022-08-10");
        assert_eq!(
            table.get_key_value("client_address").unwrap().1,
            &"street of rust"
        );
        assert_eq!(
            table.get_key_value("client_country").unwrap().1,
            &"REINO UNIDO"
        );
        assert_eq!(table.get_key_value("client_name").unwrap().1, &"iudgfs LLC");
        assert_eq!(table.get_key_value("client_nif").unwrap().1, &"12345678");
        assert_eq!(
            table.get_key_value("description").unwrap().1,
            &"An invoice for the work"
        );
        assert_eq!(table.get_key_value("nif").unwrap().1, &"87654321");
        assert_eq!(table.get_key_value("value").unwrap().1, &"1544.68");
    }

    fn get_test_table() -> [(String, String); 8] {
        [
            ("date".to_owned(), "2022-08-10".to_owned()),
            (
                "description".to_owned(),
                "An invoice for the work".to_owned(),
            ),
            ("client_country".to_owned(), "REINO UNIDO".to_owned()),
            ("client_nif".to_owned(), "12345678".to_owned()),
            ("client_name".to_owned(), "iudgfs LLC".to_owned()),
            ("client_address".to_owned(), "street of rust".to_owned()),
            ("value".to_owned(), "1544.68".to_owned()),
            ("nif".to_owned(), "87654321".to_owned()),
        ]
    }

    fn get_small_test_table() -> [(String, String); 1] {
        [("test_column".to_owned(), "test_value".to_owned())]
    }
}
