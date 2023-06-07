use std::io::Read;
use apache_avro::types::Record;
use apache_avro::AvroSchema;
use apache_avro::Schema;
use apache_avro::Reader;
use std::fs::File;
use std::path::Path;
use schema::*;
mod schema;

fn main() {
    let path = Path::new("westend_westend_20230411_westend_blocks.avro");
    let file = File::open(&path).unwrap();
    let schema_trait = Block::get_schema();
    let reader = Reader::with_schema(&schema_trait, file).unwrap();
    
    for record in reader {
        let record = record.unwrap();
        
        println!("{:?}", record);
        break;
    }
}