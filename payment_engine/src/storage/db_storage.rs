use std::error::Error;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use crate::processor::{
    record::Record,
    transaction::{Transaction, CORRECTING_TRANSACTION_TYPES},
};

use super::record_storage::RecordStorage;

pub struct DbStorage {
    db_pool: Pool<SqliteConnectionManager>,
}

impl DbStorage {
    pub fn new(db_pool: Pool<SqliteConnectionManager>) -> Self {
        Self { db_pool }
    }
}

impl RecordStorage for DbStorage {
    fn store_transaction(&mut self, txn: Transaction) -> Result<(), Box<dyn Error>> {
        let conn: PooledConnection<SqliteConnectionManager> = self.db_pool.get().unwrap();
        // normal deposit/withdrawal - has amount - insert it
        if !CORRECTING_TRANSACTION_TYPES.contains(&txn.tx_type) {
            conn.execute(
                "INSERT INTO transactions (tx_type, client, tx, disp_st, amount) values (?1, ?2, ?3, ?4, ?5)",
                [
                    &txn.tx_type.to_string(),
                    &txn.client.to_string(),
                    &txn.tx.to_string(),
                    &txn.dispute_status.to_string(),
                    &txn.amount.unwrap_or_default().to_string(),
                ],
            )?;
        } else {
            // corrective txn
            conn.execute(
                "INSERT INTO corrections (tx_type, client, tx) values (?1, ?2, ?3)",
                [
                    &txn.tx_type.to_string(),
                    &txn.client.to_string(),
                    &txn.tx.to_string(),
                ],
            )?;
        }
        Ok(())
    }

    fn get_transaction(&mut self, tx_id: u32) -> Result<Option<Transaction>, Box<dyn Error>> {
        let conn: PooledConnection<SqliteConnectionManager> = self.db_pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT * from transactions WHERE tx = :tx_id;")?;
        let records: Vec<Transaction> = stmt
            .query_map(&[(":tx_id", tx_id.to_string().as_str())], |row| {
                Ok(Transaction::from(row))
            })
            .unwrap()
            .map(|record| record.unwrap())
            .collect();

        if records.len() == 1 {
            Ok(Some(*records.first().unwrap()))
        } else {
            Ok(None)
        }
    }

    fn get_client_record(&self, client_id: u16) -> Result<Record, Box<dyn Error>> {
        let conn: PooledConnection<SqliteConnectionManager> = self.db_pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT * from records WHERE client = :client_id;")?;
        let records: Vec<Record> = stmt
            .query_map(&[(":client_id", client_id.to_string().as_str())], |row| {
                Ok(Record::from(row))
            })
            .unwrap()
            .map(|record| record.unwrap())
            .collect();

        if records.len() == 1 {
            Ok(*records.first().unwrap())
        } else {
            Ok(Record::new(client_id))
        }
    }

    fn update_record(&mut self, rec: Record) -> Result<(), Box<dyn Error>> {
        let conn: PooledConnection<SqliteConnectionManager> = self.db_pool.get().unwrap();
        if rec.locked.is_some() {
            conn.execute(
                "INSERT OR REPLACE INTO records (client, available, held, total, locked) values (?1, ?2, ?3, ?4, ?5)",
                [&rec.client.to_string(), &rec.available.to_string(), &rec.held.to_string(), &rec.total.to_string(), &rec.locked.unwrap().to_string()],
            )?;
        } else {
            conn.execute(
                "INSERT OR REPLACE INTO records (client, available, held, total) values (?1, ?2, ?3, ?4)",
                [&rec.client.to_string(), &rec.available.to_string(), &rec.held.to_string(), &rec.total.to_string()],
            )?;
        }

        Ok(())
    }

    fn update_record_and_txn(
        &mut self,
        rec: Record,
        txn: Transaction,
    ) -> Result<(), Box<dyn Error>> {
        self.update_record(rec)?;
        let mut conn: PooledConnection<SqliteConnectionManager> = self.db_pool.get().unwrap();
        let tx = conn.transaction()?;
        if rec.locked.is_some() {
            tx.execute(
                "INSERT OR REPLACE INTO records (client, available, held, total, locked) values (?1, ?2, ?3, ?4, ?5)",
                [&rec.client.to_string(), &rec.available.to_string(), &rec.held.to_string(), &rec.total.to_string(), &rec.locked.unwrap().to_string()],
            )?;
        } else {
            tx.execute(
                "INSERT OR REPLACE INTO records (client, available, held, total) values (?1, ?2, ?3, ?4)",
                [&rec.client.to_string(), &rec.available.to_string(), &rec.held.to_string(), &rec.total.to_string()],
            )?;
        }
        tx.execute(
            "UPDATE transactions SET disp_st=:disp_st WHERE tx = :tx AND tx_type = :tx_type",
            &[
                (":disp_st", &txn.dispute_status.to_string()),
                (":tx", &txn.tx.to_string()),
                (":tx_type", &txn.tx_type.to_string()),
            ],
        )?;
        tx.commit().map_err(|e| Box::from(e.to_string()))
    }

    fn write_records<W>(&self, mut wtr: csv::Writer<W>) -> Result<(), Box<dyn Error>>
    where
        W: std::io::Write + 'static,
    {
        let conn: PooledConnection<SqliteConnectionManager> = self.db_pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT * from records;")?;
        let _: Vec<Record> = stmt
            .query_map([], |row| Ok(Record::from(row)))
            .unwrap()
            .map(|row| {
                let record = row.unwrap();
                wtr.serialize(record).unwrap();
                record
            })
            .collect();
        wtr.flush()?;
        Ok(())
    }
}
