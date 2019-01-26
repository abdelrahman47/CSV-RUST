//tutorial-read-serde-04.rs
extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    //latitude: f64,
    //longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
   // Region: Option<u64>,
    //City: String,
    //Country: String,
    //AccentCity: String,
}

fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        //println!("{:?}", record.population);
        if record.population > Some(10000) && record.population < Some(1000000){
           println!("{:?}",record)
           //io::stdout().write(record)
        }
    }
    Ok(())
}
//&& record.population > Some(1000000)
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}