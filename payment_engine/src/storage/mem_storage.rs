use std::{collections::HashMap, error::Error};

use crate::processor::{
    record::Record,
    transaction::{Transaction, CORRECTING_TRANSACTION_TYPES},
};

use super::record_storage::RecordStorage;

pub struct MemStorage {
    // maps tx_id to Transaction
    transactions: HashMap<String, Transaction>,
    // maps client_id to Record
    records: HashMap<String, Record>,
}

impl MemStorage {
    pub fn new() -> MemStorage {
        Self {
            transactions: HashMap::new(),
            records: HashMap::new(),
        }
    }
}

impl Default for MemStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordStorage for MemStorage {
    fn store_transaction(&mut self, txn: Transaction) -> Result<(), Box<dyn Error>> {
        if !CORRECTING_TRANSACTION_TYPES.contains(&txn.tx_type) {
            self.transactions.insert(txn.tx.to_string(), txn);
        } // we don's store corrections in this case
        Ok(())
    }

    fn get_transaction(&mut self, tx_id: u32) -> Result<Option<Transaction>, Box<dyn Error>> {
        if let Some(txn) = self.transactions.get(&tx_id.to_string()) {
            Ok(Some(*txn))
        } else {
            Ok(None)
        }
    }

    fn get_client_record(&self, client_id: u16) -> Result<Record, Box<dyn Error>> {
        if let Some(record) = self.records.get(&client_id.to_string()) {
            Ok(*record)
        } else {
            Ok(Record::new(client_id))
        }
    }

    fn update_record(&mut self, rec: Record) -> Result<(), Box<dyn Error>> {
        self.records.insert(rec.client.to_string(), rec);
        Ok(())
    }

    fn update_record_and_txn(
        &mut self,
        rec: Record,
        txn: Transaction,
    ) -> Result<(), Box<dyn Error>> {
        self.records.insert(rec.client.to_string(), rec);
        self.transactions.insert(txn.tx.to_string(), txn);
        Ok(())
    }

    fn write_records<W>(&self, mut wtr: csv::Writer<W>) -> Result<(), Box<dyn Error>>
    where
        W: std::io::Write + 'static,
    {
        for record in self.records.values() {
            wtr.serialize(record)?
        }
        wtr.flush()?;
        Ok(())
    }
}
