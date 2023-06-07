use apache_avro::AvroSchema;
use apache_avro::from_value;
use apache_avro::Reader;
use std::fs::File;
use std::path::Path;
use schema::*;
mod schema;

use std::collections::BTreeMap;
use polars::prelude::*;

// pub fn from_vec(
//     name: &str,
//     v: Vec<<T as PolarsNumericType>::Native, Global>
// ) -> ChunkedArray<T>

fn main() {
    let path = Path::new("westend_westend_20230411_westend_blocks.avro");
    let file = File::open(&path).unwrap();
    let schema_trait = Block::get_schema();
    let reader = Reader::with_schema(&schema_trait, file).unwrap();

    let mut columns = BTreeMap::new();

    for (i, record) in reader.enumerate() {
        match record {
            Ok(value) => {
                let block = from_value::<Block>(&value).unwrap();
                // println!("block: {:?}", block);

                // let rc = Series::new("block", vec![block.relay_chain]); 
                let extrinsics = serde_json::to_string(&block.extrinsics);
                let logs = serde_json::to_string(&block.logs);
                let on_initialize = serde_json::to_string(&block.onInitialize.events);
                let on_finalize = serde_json::to_string(&block.onFinalize.events);
                                
                columns.entry("relay_chain").or_insert(vec![]).push(block.relay_chain);
                columns.entry("chain").or_insert(vec![]).push(block.chain);
                columns.entry("timestamp").or_insert(vec![]).push(block.timestamp.to_string());
                columns.entry("number").or_insert(vec![]).push(block.number);
                columns.entry("hash").or_insert(vec![]).push(block.hash);
                columns.entry("parentHash").or_insert(vec![]).push(block.parentHash);
                columns.entry("stateRoot").or_insert(vec![]).push(block.stateRoot);
                columns.entry("extrinsicsRoot").or_insert(vec![]).push(block.extrinsicsRoot);
                columns.entry("authorId").or_insert(vec![]).push(block.authorId.expect("null").to_string());
                columns.entry("finalized").or_insert(vec![]).push(block.finalized.to_string());
                columns.entry("a_extrinsics").or_insert(vec![]).push(extrinsics.expect("null"));
                columns.entry("a_onInitialize").or_insert(vec![]).push(on_initialize.expect("null"));
                columns.entry("a_onFinalize").or_insert(vec![]).push(on_finalize.expect("null"));
                columns.entry("a_logs").or_insert(vec![]).push(logs.expect("null"));

                // polars currently chokes on arrays
                // workaround would be to index into them but meh
                // the bottom line won't work
                // let s = Series::new("extrinsics", vec![block.extrinsics]); 
                // println!("s: {:?}", s);
                // df.with_column(rc).unwrap();
            }
            Err(e) => panic!("{:?}", e),
        }
        if i > 3 {
            break;
        }        
    }

    let df = DataFrame::new(
        columns.into_iter()
            .map(|(name, values)| Series::new(name, values))
            .collect::<Vec<_>>()
        ).unwrap();
    println!("df: {:#?}", df);

}
