use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn main() -> Result<(), Box<dyn Error>> {

    // Replace "your_file.txt" with the actual file name
    let file_path = "input.txt";
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);
    // Define your regular expression pattern
    let regex_pattern = r"mul\((\d+)\,(\d+)\)";
    let regex = Regex::new(regex_pattern).unwrap();
    let mut line_number = 0;
    let mut total = 0;
    let mut multiplies: Vec<(i32,i32,i32)> = Vec::new();
    let mut dos:Vec<usize> = Vec::new();
    let mut donts:Vec<usize> = Vec::new();
    let mut checkpoints:Vec<(usize, u64)> = Vec::new();


    for line in reader.lines() {
        line_number += 1;
        let uline = &line?;

        let _ = regex.captures_iter(uline).filter_map( |cap| {
            if let (Ok(first), Ok(second)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                total += first * second;
                let start = cap.get(0)?.start() as i32 + 10000 * line_number;
                //println!("{},{}, {}", first.to_string(), second.to_string(), start.to_string());
                multiplies.push((first, second, start));
                Some((first, second, start))
            } else {
                None
            }
        })
        .collect::<Vec<(i32,i32, i32)>>();

        dos.append(&mut Regex::new(r"do\(\)").unwrap().captures_iter(uline).map(|cap|{ cap.get(0).unwrap().start() + (10000 * line_number) as usize }).collect());
        donts.append(&mut  Regex::new(r"don't\(\)").unwrap().captures_iter(uline).map(|cap|{ cap.get(0).unwrap().start() + (10000 * line_number) as usize }).collect());
        
            
    }
    println!("Total result in Part 1: {}.", total.to_string());
    for item in dos {
        checkpoints.push((item, 1));
    }
    for item in donts {
        checkpoints.push((item, 2));
    }
    for item in multiplies {
        checkpoints.push((item.2 as usize, (item.0  as u64 * item.1 as u64) ));
    }
    checkpoints.sort();
    let mut new_total= 0;
    let mut good_state = true;
    for checkpoint in checkpoints {
        if checkpoint.1 == 2 {
            good_state = false;
        }
        else if checkpoint.1 == 1 {
            good_state = true;
        }
        if good_state && checkpoint.1 > 2 {
            new_total += checkpoint.1;
        }
    }
    println!("Total result in Part 2: {}.", new_total.to_string());
    Ok(())
}
