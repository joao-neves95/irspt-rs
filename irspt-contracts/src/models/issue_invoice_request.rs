use std::collections::HashMap;

use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct IssueInvoiceRequest {
    pub date: String,
    pub description: String,

    pub client_country: String,
    pub client_nif: String,
    pub client_name: String,
    pub client_address: String,

    pub value: String,
    pub nif: String,
}

impl From<&HashMap<String, String>> for IssueInvoiceRequest {
    fn from(hash_map: &HashMap<String, String>) -> Self {
        IssueInvoiceRequest {
            // TODO: Un-hardcode strings.
            date: get_hashmap_column(&hash_map, "date"),
            description: get_hashmap_column(&hash_map, "description"),
            client_country: get_hashmap_column(&hash_map, "client_country"),
            client_nif: get_hashmap_column(&hash_map, "client_nif"),
            client_name: get_hashmap_column(&hash_map, "client_name"),
            client_address: get_hashmap_column(&hash_map, "client_address"),
            value: get_hashmap_column(&hash_map, "value"),
            nif: get_hashmap_column(&hash_map, "nif"),
        }
    }
}

impl Into<HashMap<String, String>> for &IssueInvoiceRequest {
    fn into(self) -> HashMap<String, String> {
        HashMap::from([
            // TODO: Un-hardcode strings.
            ("date".to_owned(), self.date.to_owned()),
            ("description".to_owned(), self.description.to_owned()),
            ("client_country".to_owned(), self.client_country.to_owned()),
            ("client_nif".to_owned(), self.client_nif.to_owned()),
            ("client_name".to_owned(), self.client_name.to_owned()),
            ("client_address".to_owned(), self.client_address.to_owned()),
            ("value".to_owned(), self.value.to_owned()),
            ("nif".to_owned(), self.nif.to_owned()),
        ])
    }
}

fn get_hashmap_column(hash_map: &HashMap<String, String>, key: &str) -> String {
    hash_map.get(key).unwrap_or(&"".to_owned()).to_owned()
}

#[cfg(test)]
mod tests {
    use super::IssueInvoiceRequest;
    use crate::models::issue_invoice_request::get_hashmap_column;

    use std::collections::HashMap;

    #[test]
    fn get_hashmap_column_passes() -> () {
        let sut_table = HashMap::from([("test_column".to_owned(), "test_value".to_owned())]);

        let result = get_hashmap_column(&sut_table, "test_column");
        assert_eq!(result, "test_value");
    }

    #[test]
    fn from_hashmap_passes() -> () {
        let test_data: [(String, String); 8] = get_test_table();

        let model = IssueInvoiceRequest::from(&HashMap::from(test_data.clone()));
        assert_eq!(model.date, test_data[0].1);
        assert_eq!(model.description, test_data[1].1);
        assert_eq!(model.client_country, test_data[2].1);
        assert_eq!(model.client_nif, test_data[3].1);
        assert_eq!(model.client_name, test_data[4].1);
        assert_eq!(model.client_address, test_data[5].1);
        assert_eq!(model.value, test_data[6].1);
        assert_eq!(model.nif, test_data[7].1);
    }

    #[test]
    fn to_hashmap_passes() -> () {
        let test_data = IssueInvoiceRequest {
            date: "dfgadfg".to_owned(),
            client_address: "e567gtyn7uresdrgv".to_owned(),
            client_country: "uk".to_owned(),
            client_name: "adfgihuk".to_owned(),
            client_nif: "897y87987y897b".to_owned(),
            description: "5g6ep,.5e6go9u".to_owned(),
            nif: "2349853450896".to_owned(),
            value: "56474567.351".to_owned(),
        };

        let table: HashMap<String, String> = (&test_data).into();
        assert_eq!(table.get_key_value("date").unwrap().1, &"dfgadfg");
        assert_eq!(
            table.get_key_value("client_address").unwrap().1,
            &"e567gtyn7uresdrgv"
        );
        assert_eq!(table.get_key_value("client_country").unwrap().1, &"uk");
        assert_eq!(table.get_key_value("client_name").unwrap().1, &"adfgihuk");
        assert_eq!(
            table.get_key_value("client_nif").unwrap().1,
            &"897y87987y897b"
        );
        assert_eq!(
            table.get_key_value("description").unwrap().1,
            &"5g6ep,.5e6go9u"
        );
        assert_eq!(table.get_key_value("nif").unwrap().1, &"2349853450896");
        assert_eq!(table.get_key_value("value").unwrap().1, &"56474567.351");
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
}
