use std::{io, process};

use payment_engine::processor;
use processor::{tx_processor::run_with_db, utils::get_file_reader};

fn main() {
    let result = get_file_reader();
    let wtr = csv::Writer::from_writer(io::stdout());
    if let Err(err) = result {
        println!("{}", err);
        process::exit(1);
    } else {
        let reader = result.ok().unwrap();
        if let Err(err) = run_with_db(reader, wtr) {
            println!("{}", err);
            process::exit(1);
        }
    }
}
