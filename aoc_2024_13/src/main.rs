use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle:Vec<Vec<(i64,i64)>> = Vec::new();
    let pattern = Regex::new(r"X.(\d+),\sY.(\d+)").unwrap();
    let mut puzzle_piece = Vec::new();
    for line in reader.lines() {
        let string = line?;
        if string.len() != 0 {
            let _ = pattern.captures_iter(&string).filter_map( |cap| {
                if let (Ok(first), Ok(second)) = (cap[1].parse::<i64>(), cap[2].parse::<i64>()) {
                    puzzle_piece.push((first,second));
                    return Some((first,second));
                } else {return None}
            }).collect::<Vec<(i64,i64)>>();
        } if puzzle_piece.len() == 3 {
            puzzle.push(puzzle_piece);
            puzzle_piece = Vec::new();
        }
    }
    let total_cost_2:i64 = puzzle.iter().map(|x| calculate_cost_math(x)).sum();
    println!("The minimum cost of winnable prizes using math is: ${}", total_cost_2);

    Ok(())
}

fn calculate_cost_math(p: &Vec<(i64,i64)>) -> i64 {
    let (a_0, a_1, b_0, b_1, c, d) = (p[0].0,p[0].1,p[1].0,p[1].1,p[2].0 + 1E13 as i64,p[2].1 + 1E13 as i64);
    let det = (a_0 * b_1) - (b_0 * a_1);
    let x = (b_1 * c - b_0 * d) / det;
    let y = (a_0 * d - a_1 * c) / det;
    if x * a_0 + y * b_0 == c && x * a_1 + y * b_1 == d {
        println!("Cost: ${}", 3 * x + y);
        return 3 * x + y;
    }
    else {
        println!("Cost: ${}", 0);
        return 0;
    }
}
