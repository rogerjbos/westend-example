#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[macro_use] extern crate polars_core;
use polars_core::prelude::*;
use polars_lazy::prelude::*;
use polars::prelude::*;
use reqwest::blocking::Client;

use connectorx::prelude::*;
use std::convert::TryFrom;
use chrono::prelude::*;
use std::error::Error;
use polars::{df, prelude::*};
use std::env;
use std::fs::File;
use polars_io::avro::AvroReader;
// use polars_io::avro::AvroWriter;
use polars_io::SerReader;
// use polars_io::SerWriter;

// use apache_avro::Writer;
use apache_avro::Reader;
// use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;

// #[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, AvroSchema)]
// pub struct X {
//     x: bool,
// }

// lazy_static! {
//     static ref SCHEMA: Schema = X::get_schema();
// }

// pub fn decode_avro(bytes: &[u8]) -> X {
//     let value = Reader::with_schema(&SCHEMA, bytes)
//         .expect("Failed to create reader from schema")
//         // Get the first value from the Reader via the Iterator trait method
//         .next()
//         // Unwrap the Option next returns since we're expecting at least one value
//         .expect("No values in input")
//         // Unwrap the Result that the Reader provides since we expect the input to be a valid value
//         .expect("Failed to parse value");

//     from_value(&value)
//         // Unwrap the result from deserializing with serde
//         .expect("serde Deserialize failed")
// }


fn main() {

    // let mut file = File::create("output.avro").expect("couldn't create file");
    // AvroWriter::new(&mut file).finish(&mut df.unwrap()).expect("couldn't write file");
    let fname = "westend_westend_20230411_westend_blocks.avro";
    let file = File::open(fname).expect("file not found");
    println!("file found! {:?}", file);

    // ATTEMPT #1 USING POLARS - this does not work 
    // let df_avro = AvroReader::new(file).finish().expect("file not read");
   
    // ATTEMPT #2 USING APACHE_AVRO - this works
    let reader = Reader::new(file).unwrap();
    for (i, value) in reader.enumerate() {
        println!("{i}: {:?}", value.unwrap());
        if i > 2{
            break;
        }
    }

}
