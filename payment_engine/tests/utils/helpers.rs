use std::fs::File;

use csv::{Reader, Trim};

pub fn get_csv_reader(file_name: &str) -> Reader<File> {
    let file_in: File = File::open(format!("./resources/{}.csv", file_name)).unwrap();
    csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(file_in)
}
