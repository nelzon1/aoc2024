use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Define your regular expression pattern
    let regex_pattern = r"(\d+)\s+(\d+)";
    let regex = Regex::new(regex_pattern)?;

    let mut left_column: Vec<u64> = Vec::new();
    let mut right_column: Vec<u64> = Vec::new();

    // Iterate over the lines
    for line in reader.lines() {
        let orig_line = line?;
        // Iterate over all matches on the line
        if let Some(captures) = regex.captures(&orig_line) {
            // Access captured groups using captures
            let num1 = captures[1].parse::<u64>().unwrap();
            let num2 = captures[2].parse::<u64>().unwrap();
            left_column.push(num1);
            right_column.push(num2);
        }
    }

    left_column.sort();
    right_column.sort();
    let mut total_distance: u64 = 0;

    for n in 0..left_column.len() {
        total_distance += ((left_column[n]) as i64 - (right_column[n]) as i64).abs() as u64;
    }

    let mut similarity_score: u64 = 0;

    for n in 0..left_column.len() {
        similarity_score +=  left_column[n] * right_column.iter().filter(|&x| *x == left_column[n]).count() as u64;
    }

    println!("The total distance is {}" , total_distance.to_string());
    println!("The total similarity score is {}" , similarity_score.to_string());

    Ok(())
}
