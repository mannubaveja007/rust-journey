use std::{error::Error, fs::File};

use csv::Reader;

fn read_csv(path: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(Reader::from_reader(file))
}

fn write_csv(path: &str, records: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    let mut csv = csv::Writer::from_path(path)?;

    for record in records {
        csv.write_record(record)?;
    }
    csv.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Printing the information read from CSV file!");
    let file = File::open("Morgan Stanly.csv").expect("Dear check your CSV file");
    let mut csv = csv::Reader::from_reader(file);
    for result in csv.records() {
        let record = result?;
        println!("{:?}", record);
    }

    let bruh = Reader::from_path("/Users/mannubaveja/temp/rust/learn_csv_read/Morgan Stanly.csv")?;

    println!("{:?}", bruh);
    Ok(())
}
