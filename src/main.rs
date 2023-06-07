#![allow(dead_code)] 
#![allow(unused_imports)] 
#![allow(unused_variables)] 
use apache_avro::AvroSchema;
use clap::Parser;
use glob::glob;
use log::debug;
use std::fs;
use std::fs::File;
use apache_avro::Writer;
use apache_avro::Reader;
use apache_avro::from_value;

#[derive(Debug)]
pub struct Block {
    relay_chain: String,
    chain: String,
    timestamp: String,
    number: String,
    hash: String,
    parent_hash: String,
    state_root: String,
    extrinsics_root: String,
    author_id: String,
    finalized: String,
    extrincis: String,
    on_initialize: String,
    on_finalize: String,
    log: String,
}

fn main() {

    let fname = "westend_westend_20230411_westend_blocks.avro";
    let file = File::open(fname).expect("file not found");
    println!("file found! {:?}", file);

    // ATTEMPT #1 USING POLARS - this does not work 
    // let df_avro = AvroReader::new(file).finish().expect("file not read");
   
    // ATTEMPT #2 USING APACHE_AVRO - this works
    let mut reader = Reader::new(file).unwrap();

    let record = reader.next().unwrap();
    // println!("Record: {:?}", record.unwrap());
    let test = from_value::<Block>(&record.unwrap());
    
    for (i, value) in reader.enumerate() {
        let v = value.unwrap();
        println!("\n##### {i} :{:?}", v);
        // println!("{:#?}", from_value::<Block>(&value.unwrap()));
        if i > 0{
            break;
        }
    }
}
