use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "input.txt";
    //let debug_file_path: &str = "debug.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<u32>> = Vec::new();

    // Iterate over the lines
    for line in reader.lines() {
        let orig_line = line?;
        // Iterate over all matches on the line
        reports.push(orig_line.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>());
    }

    let valid_reports = reports.iter().filter(|x| validate_report(x)).count();
    let valid_reports_dampen = reports.iter().filter(|x| validate_report_dampen(x)).count();
    println!("Valid repots: {}.", valid_reports.to_string());
    println!("Valid dampened repots: {}.", valid_reports_dampen.to_string());
    
    Ok(())
}

fn validate_report(report:&Vec<u32>) -> bool {
    let mut increasing:bool = true;
    let mut last_value:u32 = 0;
    let mut count = 0;
    for number in report {
        count += 1;
        if count == 1 {
            last_value = *number;
            continue;
        }
        if count == 2 {
            increasing = *number as i32 - last_value as i32 > 0
        }
        if !validate_rules_1(last_value, *number, increasing){
            return false;
        }
        last_value = *number;
    }
    return true;
}

fn validate_rules_1(a:u32, b:u32, increasing: bool) -> bool {
    let diff:i32 = b as i32 - a as i32;
    let diff_limited = diff.abs() <= 3 && diff.abs() >= 1;
    //println!("{}, {}, {}", a.to_string(), b.to_string(), ((increasing && diff > 0) || (!increasing && diff < 0)) && diff_limited);
    return ((increasing && diff > 0) || (!increasing && diff < 0)) && diff_limited;
}

fn validate_report_dampen(report:&Vec<u32>) -> bool {
    if validate_report(report) {
        return true;
    }
    for index in 0..report.len() {
        let mut trial_report = report.clone();
        trial_report.remove(index);
        if validate_report(&trial_report) {
            return true;
        }
    }
    return false;
}