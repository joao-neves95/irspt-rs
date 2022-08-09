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

impl From<HashMap<&str, String>> for IssueInvoiceRequest {
    fn from(hash_map: HashMap<&str, String>) -> Self {
        IssueInvoiceRequest {
            // TODO: Un-hardcode strings.
            date: get_hashmap_column("date", &hash_map),
            description: get_hashmap_column("description", &hash_map),
            client_country: get_hashmap_column("client_country", &hash_map),
            client_nif: get_hashmap_column("client_nif", &hash_map),
            client_name: get_hashmap_column("client_name", &hash_map),
            client_address: get_hashmap_column("client_address", &hash_map),
            value: get_hashmap_column("value", &hash_map),
            nif: get_hashmap_column("nif", &hash_map),
        }
    }
}

fn get_hashmap_column(key: &str, hash_map: &HashMap<&str, String>) -> String {
    hash_map.get(key).unwrap_or(&"".to_owned()).to_owned()
}
