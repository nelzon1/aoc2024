
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let blinks =75;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle:Vec<u64> = Vec::new();
    for line in reader.lines() {
        puzzle = line?.split_whitespace().into_iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    }
    let mut rock_vec:Vec<u64> = Vec::new();
    for rock in &puzzle {
        rock_vec.push(*rock);
        print!("{}, ", rock);
    }
    println!("{}","");

    let mut score:u64 = 0;
    let mut cache:HashMap<(u64,u32),u64> = HashMap::new();

    for rock in rock_vec{
    score +=  score_number(rock, blinks,0,1 ,&mut cache);
    }

    //println!("There are {} rocks after {} blinks.", rock_vec.len(), blinks );
    println!("There are {} rocks after {} blinks", score ,blinks);
    Ok(())
}

fn apply_rules(input: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();

    if input == 0 {
        output.push(1);
    }
    else if even_digits(input) {
        let first = input.to_string().chars().into_iter().take(input.to_string().len()/2).collect::<String>().parse::<u64>().unwrap();
        let last = input.to_string().chars().into_iter().rev().take(input.to_string().len()/2).collect::<String>().chars().into_iter().rev().collect::<String>().parse::<u64>().unwrap();
        output.push(first);
        output.push(last);
    }
    else{
        output.push(input * 2024);
    }
    return output;
}

fn even_digits(input:u64) -> bool {
    return input.to_string().chars().count() % 2 == 0;
}

fn score_number(input: u64, blinks: u32, curblinks:u32, mut curscore:u64, cache: &mut HashMap<(u64,u32),u64>) -> u64 {
    if cache.contains_key(&(input,curblinks)) {
        return *cache.get(&(input,curblinks)).unwrap();
    }
    if curblinks == blinks {return curscore;}
    if input == 0 {
        let result =  score_number(1,blinks, curblinks + 1, curscore, cache);
        cache.insert((input, curblinks), result);
        return result;
    }
    else if even_digits(input) {
        //curscore += 1;
        let first = input.to_string().chars().into_iter().take(input.to_string().len()/2).collect::<String>().parse::<u64>().unwrap();
        let last = input.to_string().chars().into_iter().rev().take(input.to_string().len()/2).collect::<String>().chars().into_iter().rev().collect::<String>().parse::<u64>().unwrap();
        let result = score_number(first, blinks, curblinks + 1,curscore, cache) + score_number(last,blinks,  curblinks + 1,1, cache);
        cache.insert((input, curblinks), result);
        return result;
    }
    else{
        let result = score_number(input * 2024,blinks,  curblinks + 1,curscore, cache);
        cache.insert((input, curblinks), result);
        return result; 
    }
}