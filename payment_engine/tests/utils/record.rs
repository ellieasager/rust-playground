#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Record {
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Record {
    pub fn new(client: u16, available: f64, held: f64, total: f64, locked: bool) -> Self {
        Record {
            client,
            available,
            held,
            total,
            locked,
        }
    }
}
