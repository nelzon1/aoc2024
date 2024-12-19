
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug3.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle:Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        puzzle.push( line?.chars().into_iter().map(|x| x.to_digit(10).unwrap() ).collect::<Vec<u32>>());
    }
    
    Ok(())
}
