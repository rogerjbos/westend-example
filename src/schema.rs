#![allow(non_snake_case)] // So that the compiler doesn't throw a warning because of snake case.
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Signer {
    id: String,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Signature {
    signature: String,

    #[serde(deserialize_with = "string_or_struct")]
    signer: Signer,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Event {
    pub method: Method,
    pub data: String,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Era {
    immortalEra: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Method {
    pub pallet: String,
    pub method: String,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Extrinsic {
    pub method: Method,
    signature: Option<Signature>,
    nonce: Option<String>,
    pub args: String,
    tip: Option<String>,
    hash: String,
    info: String,
    era: Era,
    pub events: Vec<Event>,
    success: bool,
    paysFee: bool,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Block {
    // #[serde(skip_deserializing)]
    pub relay_chain: String,
    // #[serde(skip_deserializing)]
    pub chain: String,
    // #[serde(skip_deserializing)]
    pub timestamp: i64,

    number: String,
    hash: String,
    parentHash: String,
    stateRoot: String,
    extrinsicsRoot: String,
    authorId: Option<String>,
    finalized: bool,
    pub extrinsics: Vec<Extrinsic>,
    onInitialize: SystemEvent,
    onFinalize: SystemEvent,
    logs: Vec<Log>,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Now {
    pub now: String,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct SystemEvent {
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug, Serialize, apache_avro::AvroSchema)]
pub struct Log {
    #[serde(alias = "type")]
    log_type: String,
    index: String,
    value: String,
}

impl FromStr for Signer {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signer { id: s.to_string() })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
