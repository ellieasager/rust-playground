use csv::{Reader, Trim};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serializer;
use std::result::Result;
use std::{env, error::Error, fs::File};

pub fn get_file_reader() -> Result<Reader<File>, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected a file name, but got none")),
        Some(file_path) => {
            let file_in = File::open(file_path)?;
            let rdr = csv::ReaderBuilder::new()
                .flexible(true)
                .trim(Trim::All)
                .from_reader(file_in);
            Ok(rdr)
        }
    }
}

pub fn create_pool() -> Result<Pool<SqliteConnectionManager>, Box<dyn Error>> {
    let manager = SqliteConnectionManager::file("records.db");
    let pool = Pool::builder().max_size(5).build(manager)?;
    init_db(&pool)?;
    Ok(pool)
}

fn init_db(db_pool: &Pool<SqliteConnectionManager>) -> Result<(), Box<dyn Error>> {
    let conn: PooledConnection<SqliteConnectionManager> = db_pool.get().unwrap();

    conn.execute("drop table if exists transactions", [])?;
    conn.execute("drop table if exists corrections", [])?;
    conn.execute("drop table if exists records", [])?;

    conn.execute(
        "create table transactions (
           tx integer primary key,
           client integer,
           tx_type text,
           disp_st text,
           amount real
       )",
        [],
    )?;
    conn.execute(
        "create table corrections (
           tx integer,
           client integer,
           tx_type text
       )",
        [],
    )?;
    conn.execute(
        "create table records (
         client integer primary key,
         available real,
         held real,
         total real,
         locked integer
     )",
        [],
    )?;
    Ok(())
}

pub fn int_to_bool<S>(value: &Option<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match value {
        Some(1) => "true",
        _ => "false",
    };
    serializer.serialize_str(s)
}
