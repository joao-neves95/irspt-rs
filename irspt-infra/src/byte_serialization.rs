use anyhow::{Ok, Result};
use bytecheck::CheckBytes;
use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer,
    validation::validators::DefaultValidator, Archive, Deserialize, Serialize,
};
use sled::IVec;

pub fn serialize_to_bytes<TModel>(model: &TModel) -> Result<IVec>
where
    TModel: Serialize<AllocSerializer<512>>,
{
    Ok(sled::IVec::from(rkyv::to_bytes::<_, 512>(model)?.as_ref()))
}

pub fn deserialize_from_bytes<'a, TModel>(raw_result: &'a IVec) -> Result<TModel>
where
    TModel: Archive,
    TModel::Archived:
        'a + CheckBytes<DefaultValidator<'a>> + Deserialize<TModel, SharedDeserializeMap>,
{
    match rkyv::from_bytes::<TModel>(raw_result) {
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

    #[test]
    fn de_serialize_struct_passes() {
        let model = IssueInvoiceRequest {
            date: "2020-08-01".to_owned(),
            description: "the service I did".to_owned(),
            client_nif: "123".to_owned(),
            client_name: "test inc.".to_owned(),
            client_country: "usa".to_owned(),
            client_address: "street 1".to_owned(),
            value: "12345".to_owned(),
            nif: "321".to_owned(),
        };

        let raw_bytes_res = serialize_to_bytes(&model);
        assert!(raw_bytes_res.is_ok());

        let deserialized_model_res =
            deserialize_from_bytes::<IssueInvoiceRequest>(&raw_bytes_res.unwrap());
        assert!(deserialized_model_res.as_ref().is_ok());

        let deserialized_model = deserialized_model_res.as_ref().unwrap();
        assert!(deserialized_model.value == model.value);
        assert!(deserialized_model.date == model.date);
        assert!(deserialized_model.description == model.description);
        assert!(deserialized_model.nif == model.nif);
        assert!(deserialized_model.client_name == model.client_name);
        assert!(deserialized_model.client_nif == model.client_nif);
    }

    #[test]
    fn de_serialize_int_passes() {
        let primitive_value: u8 = 21;

        let raw_bytes_res = serialize_to_bytes(&primitive_value);
        assert!(raw_bytes_res.is_ok());

        let deserialized_value_res = deserialize_from_bytes::<u8>(&raw_bytes_res.unwrap());
        assert!(deserialized_value_res.as_ref().is_ok());
        assert!(deserialized_value_res.unwrap() == primitive_value);
    }
}
