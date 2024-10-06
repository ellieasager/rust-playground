use std::fs::File;
use std::{error::Error, io};

use std::result::Result;

use csv::{Reader, Writer};

use crate::storage::{
    db_storage::DbStorage, mem_storage::MemStorage, record_storage::RecordStorage,
};

use super::utils::create_pool;
use super::{record::Record, transaction::Transaction};

pub fn run_in_mem<W>(file: Reader<File>, wtr: Writer<W>) -> Result<(), Box<dyn Error>>
where
    W: io::Write + 'static,
{
    let mem_storage = MemStorage::new();
    run(file, wtr, mem_storage)
}

pub fn run_with_db<W>(file: Reader<File>, wtr: Writer<W>) -> Result<(), Box<dyn Error>>
where
    W: io::Write + 'static,
{
    let db_pool = create_pool()?;
    let db_storage = DbStorage::new(db_pool.clone());
    run(file, wtr, db_storage)
}

pub fn run<W>(
    mut rdr: Reader<File>,
    wtr: Writer<W>,
    mut record_storage: impl RecordStorage,
) -> Result<(), Box<dyn Error>>
where
    W: io::Write + 'static,
{
    // read and process
    for txn in rdr.deserialize() {
        let txn: Transaction = txn?;
        let current_client_data: Record = record_storage.get_client_record(txn.client)?;
        let mut txn_to_check: Option<Transaction> = None;
        if let Some(tx_id) = txn.tx_id_to_check() {
            txn_to_check = record_storage.get_transaction(tx_id)?;
        }
        _ = record_storage.store_transaction(txn);
        let (maybe_record, maybe_txn) = txn.process(&current_client_data, txn_to_check)?;

        _ = match (maybe_record, maybe_txn) {
            (Some(record), Some(txn)) => record_storage.update_record_and_txn(record, txn),
            (Some(record), None) => record_storage.update_record(record),
            _ => Ok(()), // this shouldn't happen,
        };
    }

    // print back
    record_storage.write_records(wtr)
}

#[cfg(test)]
mod tests {

    use std::fs::File;

    use csv::{Reader, Trim, Writer};

    use super::run;
    use crate::{processor::record::Record, storage::record_storage::MockRecordStorage};
    // cargo test --package payment_engine --bin payment_engine -- processor::tx_processor::tests::test_run --exact --show-output

    #[test]
    fn test_run() {
        let reader = open_test_file("t1");
        let mut record_storage = MockRecordStorage::new();
        let wtr = Writer::from_path("./resources/tmp.csv").ok().unwrap();

        record_storage
            .expect_get_client_record()
            .once()
            .returning(move |_| Ok(Record::new(1)));
        record_storage
            .expect_store_transaction()
            .once()
            .returning(|_| Ok(()));
        record_storage
            .expect_update_record()
            .once()
            .returning(|_| Ok(()));
        record_storage
            .expect_write_records()
            .once()
            .returning(|_: Writer<File>| Ok(()));
        _ = run(reader, wtr, record_storage);
    }

    fn open_test_file(file_name: &str) -> Reader<File> {
        let file_in: File = File::open(format!("./resources/{}.csv", file_name)).unwrap();
        csv::ReaderBuilder::new()
            .flexible(true)
            .trim(Trim::All)
            .from_reader(file_in)
    }
}
