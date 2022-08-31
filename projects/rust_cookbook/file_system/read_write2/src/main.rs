use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::time::Instant;

fn main() -> Result<(), Error> {

    let start = Instant::now();
    let path = "majestic_million.csv";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    let duration = start.elapsed();
    println!("Time elapsed in read 1M rows of csv file is {:?}", duration);
    
    Ok(())

}
/*
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
*/
