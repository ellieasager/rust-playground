use std::{error::Error, io};

use crate::processor::{record::Record, transaction::Transaction};
use csv::Writer;
use mockall::mock;

pub trait RecordStorage {
    fn store_transaction(&mut self, t: Transaction) -> Result<(), Box<dyn Error>>;
    fn get_transaction(&mut self, tx_id: u32) -> Result<Option<Transaction>, Box<dyn Error>>;
    fn get_client_record(&self, client_id: u16) -> Result<Record, Box<dyn Error>>;
    fn update_record(&mut self, r: Record) -> Result<(), Box<dyn Error>>;
    fn update_record_and_txn(&mut self, r: Record, t: Transaction) -> Result<(), Box<dyn Error>>;
    fn write_records<W>(&self, wtr: Writer<W>) -> Result<(), Box<dyn Error>>
    where
        W: io::Write + 'static;
}

mock! {
    pub RecordStorage {}
    impl RecordStorage for RecordStorage {
        fn store_transaction(&mut self, t: Transaction) -> Result<(), Box<dyn Error>>;
        fn get_transaction(&mut self, tx_id: u32) -> Result<Option<Transaction>, Box<dyn Error>>;
        fn get_client_record(&self, client_id: u16) -> Result<Record, Box<dyn Error>>;
        fn update_record(&mut self, r: Record) -> Result<(), Box<dyn Error>>;
        fn update_record_and_txn(&mut self, r: Record, t: Transaction) -> Result<(), Box<dyn Error>>;
        fn write_records<W>(&self, wtr: Writer<W>) -> Result<(), Box<dyn Error>> where W: io::Write + 'static;
    }
}
