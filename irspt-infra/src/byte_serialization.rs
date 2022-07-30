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
    Ok(sled::IVec::from(
        rkyv::to_bytes::<_, 512>(model)?.as_slice(),
    ))
}

pub fn deserialize_from_bytes<'a, TModel>(raw_result: &'a IVec) -> TModel
where
    TModel: Archive,
    TModel::Archived:
        'a + CheckBytes<DefaultValidator<'a>> + Deserialize<TModel, SharedDeserializeMap>,
{
    rkyv::from_bytes::<TModel>(raw_result).unwrap()
}
