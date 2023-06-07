use apache_avro::AvroSchema;
use apache_avro::from_value;
use apache_avro::Reader;
use std::fs::File;
use std::path::Path;
use schema::*;
mod schema;

use polars::prelude::*;

fn main() {
    let path = Path::new("westend_westend_20230411_westend_blocks.avro");
    let file = File::open(&path).unwrap();
    let schema_trait = Block::get_schema();
    let reader = Reader::with_schema(&schema_trait, file).unwrap();

    // Create an empty DataFrame
    let mut df = DataFrame::default();
    
    for record in reader {
        match record {
            Ok(value) => {
                let block = from_value::<Block>(&value).unwrap();

                let s = Series::new("block", vec![block.relay_chain]); 

                df.with_column(s).unwrap();
                
            }
            Err(e) => panic!("{:?}", e),
        }

        // Print the DataFrame
        println!("{:?}", df);
        
        break;
    }
}