use std::io::{self, Write};

fn main() {
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

    let sum = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, sum);
}
