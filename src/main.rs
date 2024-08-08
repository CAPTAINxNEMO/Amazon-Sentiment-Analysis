use std::{error::Error, fs::{File, OpenOptions}, io::{self, Write}, path::Path};
use csv::{Writer, Reader};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Reviews {
    review: String,
    score: f32,
    sentiment: String,
}

fn scraper <P: AsRef<Path>, S: Into<String>>(filename: P, link: S) -> Result<(), Box<dyn Error>> {
    let link = link.into();
    println!("link: {}", link);

    let file = OpenOptions::new().write(true).truncate(true).open(filename)?;
    let mut wtr = Writer::from_writer(file);

    let reviews_list = vec![
        Reviews {
            review: "This is a great product!".to_string(),
            score: 1.0,
            sentiment: "Positive".to_string(),
        },
        Reviews {
            review: "This is a bad product!".to_string(),
            score: -1.0,
            sentiment: "Negative".to_string(),
        },
        Reviews {
            review: "This is an okay product!".to_string(),
            score: 0.3,
            sentiment: "Positive".to_string(),
        },
        Reviews {
            review: "I am a boy.".to_string(),
            score: 0.0,
            sentiment: "Neutral".to_string(),
        },
        Reviews {
            review: "I am a good boy.".to_string(),
            score: 1.0,
            sentiment: "Positive".to_string(),
        },
        Reviews {
            review: "I am a bad boy.".to_string(),
            score: -1.0,
            sentiment: "Neutral".to_string(),
        },
    ];
    
    for review in &reviews_list {
        wtr.serialize(review)?;
    }
    Ok(())
}

fn analyzer <P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
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
    print!("Enter product link: ");
    io::stdout().flush().unwrap();
    let mut link = String::new();
    io::stdin().read_line(&mut link).expect("Failed to read input");
    let link: String = link.trim().parse().expect("Please enter a valid link.");

    let filename = "Reviews.csv";
    let _ = scraper(filename, link);
    analyzer(filename)
}
