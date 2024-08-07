use std::io::{self, Write};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Reviews {
    review: String,
    score: f32,
    sentiment: String,
}

fn analyzer<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);
    let mut record_count = 0;
    let mut total_score = 0.0;

    for result in rdr.deserialize() {
        let record: Reviews = result?;
        println!("Review    : {}\nScore     : {}\nSentiment : {}\n{}", record.review, record.score, record.sentiment, String::from("=").repeat(50));

        record_count += 1;
        total_score += record.score;
    }

    if record_count > 0 {
        let average_score = total_score / record_count as f32;
        println!("Total number of reviews : {}\nAverage score           : {}", record_count, average_score);
        if average_score >= 0.05 {
            println!("Sentiment               : Positive");
        } else if average_score <= -0.05 {
            println!("Sentiment               : Negative");
        } else {
            println!("Sentiment               : Neutral");
        }
    } else {
        println!("No reviews processed.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("Enter num1: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f32 = num1.trim().parse().expect("Please enter a valid number.");

    print!("Enter num2: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f32 = num2.trim().parse().expect("Please enter a valid number.");

    println!("The sum of {} and {} is {}.\n{}", num1, num2, num1 + num2, String::from("=").repeat(50));

    let filename = "Reviews.csv";
    analyzer(filename)
}
