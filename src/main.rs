use std::io::{self, Write};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;

fn webpage_parser<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        
        let review = &record[0];
        let score: f64 = record[1].parse()?;
        let sentiment = &record[2];

        println!("Record {}:\nReview: {}\nScore: {}\nSentiment: {}\n", i + 1, review, score, sentiment);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("Enter num1: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number.");

    print!("Enter num2: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number.");

    println!("The sum of {} and {} is {}.", num1, num2, num1 + num2);

    let filename = "Reviews.csv";
    webpage_parser(filename)
}