//tutorial-read-serde-04.rs
extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::fs::File;

use std::process;
use std::time::Instant;

fn run() -> Result<(), Box<dyn Error>> {
    
    let path = "majestic_million.csv";
    let file = File::open(path)?;
    
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize(){
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
        
}
#[derive(Debug, Deserialize)]
struct Record {
    GlobalRank: i64,
    TldRank: i64,
    Domain : String,
    TLD: String,
    RefSubNets: i64,
    RefIPs: i64,
    IDN_Domain: String,
    IDN_TLD: String,
    PrevGlobalRank: i64,
    PrevTldRank: i64,
    PrevRefSubNets: i64,
    PrevRefIPs: i64
}
/*
global_rank : i64,
tld_rank: i64,
domain: String,
tld: String,
ref_sub_nets: i64,
ref_ips: i64,
idn_domain: String,
idn_tld: String,
prev_global_rank: i64,
prev_tld_rank: i64,
prev_ref_sub_nets: i64,
prev_ref_ips: i64,
 */
fn main() {
    let start = Instant::now();
    
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    
    let duration = start.elapsed();
    println!("Time elapsed in read 1M rows of csv file is {:?}", duration);

}
/*
let conn = Connection::open("workshop.db").unwrap();
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
              PRAGMA synchronous = 0;
              PRAGMA cache_size = 1000000;
              PRAGMA locking_mode = EXCLUSIVE;
              PRAGMA temp_store = MEMORY;",
    )
    .expect("PRAGMA");
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS domain (
            GlobalRank int,
            TldRank int,
            Domain varchar(255),
            TLD varchar(255),
            RefSubNets int,
            RefIPs int,
            IDN_Domain varchar(255),
            IDN_TLD varchar(255),
            PrevGlobalRank int,
            PrevTldRank int,
            PrevRefSubNets int,
            PrevRefIPs int
        );",
        [],
    )
    .unwrap();
    faker(conn, 1_000_000)
fn main() -> Result<(), Error> {


    let start = Instant::now();
    
    let path = "majestic_million.csv";

    
    let duration = start.elapsed();
    println!("Time elapsed in read 1M rows of csv file is {:?}", duration);
    
    Ok(())

}

use rusqlite::{params, Connection, Transaction};

mod common;

fn faker_wrapper(mut conn: Connection, count: i64) {
    let tx = conn.transaction().unwrap();
    faker(&tx, count);
    tx.commit().unwrap();
}

fn faker(tx: &Transaction, count: i64) {
    let mut stmt_with_area = tx
        .prepare_cached("INSERT INTO user VALUES (?, ?, ?, ?)")
        .unwrap();
    let mut stmt = tx
        .prepare_cached("INSERT INTO user VALUES (?, NULL, ?, ?)")
        .unwrap();
    let mut pk: i64 = 1;
    for _ in 0..count {
        let with_area = common::get_random_bool();
        let age = common::get_random_age();
        let is_active = common::get_random_active();
        if with_area {
            let area_code = common::get_random_area_code();
            stmt_with_area
                .execute(params![pk, area_code, age, is_active])
                .unwrap();
        } else {
            stmt.execute(params![pk, age, is_active]).unwrap();
        }
        pk += 1;
    }
}

fn main() {
    let conn = Connection::open("basic_prep.db").unwrap();
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
              PRAGMA synchronous = 0;
              PRAGMA cache_size = 1000000;
              PRAGMA locking_mode = EXCLUSIVE;
              PRAGMA temp_store = MEMORY;",
    )
    .expect("PRAGMA");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
                id INTEGER not null primary key,
                area CHAR(6),
                age INTEGER not null,
                active INTEGER not null)",
        [],
    )
    .unwrap();
    faker_wrapper(conn, 100_000_000)
}

*/