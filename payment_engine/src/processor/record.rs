use rusqlite::Row;
use serde::Serialize;

use super::utils::int_to_bool;

#[derive(Debug, Serialize, Copy, Clone, PartialEq)]
pub struct Record {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    #[serde(serialize_with = "int_to_bool")]
    pub locked: Option<u8>,
}

impl Record {
    pub fn new(client_id: u16) -> Self {
        Record {
            client: client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: None,
        }
    }
}

impl From<&Row<'_>> for Record {
    fn from(row: &Row) -> Self {
        Record {
            client: row.get(0).unwrap(),
            available: row.get(1).unwrap(),
            held: row.get(2).unwrap(),
            total: row.get(3).unwrap(),
            locked: row.get(4).unwrap(),
        }
    }
}
