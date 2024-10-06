use csv::{Trim, Writer};
use std::fs::File;

use crate::{
    cases::FILE_OUT_NAME,
    utils::{helpers::get_csv_reader, record::Record},
};
use payment_engine::{
    processor::{tx_processor::run, utils::create_pool},
    storage::db_storage::DbStorage,
};

pub fn run_test(input_file_name: &str, expected_results: Vec<Record>) {
    let reader = get_csv_reader(input_file_name);
    let db_pool = create_pool().unwrap();
    let record_storage = DbStorage::new(db_pool.clone());
    let wtr = Writer::from_path(FILE_OUT_NAME).ok().unwrap();
    _ = run(reader, wtr, record_storage);

    let file_out = File::open(FILE_OUT_NAME).unwrap();
    let mut result_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(file_out);
    let mut records: Vec<Record> = Vec::new();
    for result in result_reader.deserialize() {
        let record: Record = result.unwrap();
        records.push(record);
    }

    assert_eq!(expected_results, records);
}
