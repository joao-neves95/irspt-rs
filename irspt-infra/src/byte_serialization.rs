use anyhow::{Ok, Result};
use bytecheck::CheckBytes;
use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer,
    validation::validators::DefaultValidator, Archive, Deserialize, Serialize,
};
use sled::IVec;

pub fn serialize_to_bytes<T>(value: &T) -> Result<IVec>
where
    T: Serialize<AllocSerializer<512>>,
{
    Ok(sled::IVec::from(rkyv::to_bytes::<T, 512>(value)?.as_ref()))
}

pub fn deserialize_from_bytes<'a, T>(raw_result: &'a IVec) -> Result<T>
where
    T: Archive,
    T::Archived: 'a + CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    match rkyv::from_bytes::<T>(raw_result) {
        Result::Ok(result) => Ok(result),
        Err(_) => Err(anyhow::anyhow!(
            "Byte deserialization error. Invalid struct."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::deserialize_from_bytes;
    use super::serialize_to_bytes;
    use irspt_contracts::models::IssueInvoiceRequest;

    use std::collections::HashMap;

    #[test]
    fn de_serialize_int_passes() {
        let primitive_value: u8 = 21;

        let raw_bytes_res = serialize_to_bytes(&primitive_value);
        assert!(raw_bytes_res.is_ok());

        let deserialized_value_res = deserialize_from_bytes::<u8>(&raw_bytes_res.unwrap());
        assert!(deserialized_value_res.as_ref().is_ok());
        assert!(deserialized_value_res.unwrap() == primitive_value);
    }

    #[test]
    fn de_serialize_hashmap_passes() {
        let table = HashMap::from([
            ("1".to_owned(), "4n758y_rg897ys".to_owned()),
            ("2".to_owned(), "97b6gng$&€€$$$".to_owned()),
            ("3".to_owned(), "r9set_8".to_owned()),
        ]);

        let raw_bytes_res = serialize_to_bytes(&table);
        assert!(raw_bytes_res.is_ok());

        let deserialized_value_res =
            deserialize_from_bytes::<HashMap<String, String>>(&raw_bytes_res.unwrap());
        assert!(deserialized_value_res.as_ref().is_ok());
        assert!(
            deserialized_value_res
                .unwrap()
                .get_key_value("2")
                .unwrap()
                .1
                == "97b6gng$&€€$$$"
        );
    }

    #[test]
    fn de_serialize_hashmap_in_struct_passes() {
        let mut model = IssueInvoiceRequest::new_empty();
        model.set_date("2020-08-01".to_owned());
        model.set_description("the service I did".to_owned());
        model.set_client_nif("123".to_owned());
        model.set_client_name("test inc.".to_owned());
        model.set_client_country("usa".to_owned());
        model.set_client_address("street 1".to_owned());
        model.set_value("12345".to_owned());
        model.set_nif("321".to_owned());

        let raw_bytes_res = serialize_to_bytes::<HashMap<String, String>>(&model.data);
        assert!(raw_bytes_res.is_ok());

        let deserialized_model_res =
            deserialize_from_bytes::<HashMap<String, String>>(&raw_bytes_res.unwrap());
        assert!(deserialized_model_res.is_ok());

        let deserialized_model = IssueInvoiceRequest::new(deserialized_model_res.unwrap());
        assert!(deserialized_model.get_value() == model.get_value());
        assert!(deserialized_model.get_date() == model.get_date());
        assert!(deserialized_model.get_description() == model.get_description());
        assert!(deserialized_model.get_nif() == model.get_nif());
        assert!(deserialized_model.get_client_name() == model.get_client_name());
        assert!(deserialized_model.get_client_nif() == model.get_client_nif());
    }
}
