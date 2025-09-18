use std::io::{self, Write};

fn main() {
    println!("Enter integers, one per line. Empty line to finish.");  // added semicolon

    let nums: Vec<i32> = Vec::new();  // added semicolon

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { return; }

        let trimmed = input.trim();

        if trimmed.is_empty(){  // added squiggly bracket
          break; 

        match trimmed.parse::<i32>() {
            Ok(n) => nums.push(n),
            Err(_) => println!("Please enter a valid integer."),
        }
    }

    if nums.is_empty() {
        println!("No numbers entered.");
    } else {
        let sum: i32 = nums.iter().sum();
        let avg = sum as f64 / nums.len() as f64;
        println!("Count = {nums.len()}, Sum = {sum}, Average = {avg:.2}");
    }
}
