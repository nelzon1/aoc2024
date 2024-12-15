use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn test_eqn(total:u64, current:u64, mut remaining:Vec<u64>) -> bool {
    if remaining.len() == 0 {
        return current == total;
    }
    let next_val = remaining.remove(0);
    return test_eqn(total, current + next_val, remaining.clone()) 
    ||  test_eqn(total, current * next_val, remaining.clone()) 
    || test_eqn(total, concat_number(current, next_val), remaining.clone());
}   

fn concat_number(a:u64, b:u64) -> u64 {
    let test_str = a.to_string() + &b.to_string();
    if let Ok(result) = test_str.parse::<u64>() {
        return result;
    }
    else {
        return 0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle: Vec<String> = Vec::new();
    for line in reader.lines() {
        let result = line?;
         puzzle.push(result);
    }

    let mut good_eqns = 0;
    let mut total_calibration = 0;
    let mut good_eqns_2 = 0;
    let mut total_calibration_2 = 0;

    for eqn in puzzle {
        let total:u64;
        let mut pieces:Vec<u64>;

        total = eqn.split(':').into_iter().nth(0).unwrap().parse::<u64>().unwrap();
        pieces = eqn.split(':').into_iter().nth(1).unwrap().split_whitespace().into_iter().map(|x| x.parse::<u64>().unwrap() ).collect::<Vec<u64>>();
        //part 1
        let index = (pieces.len() - 1) as u32;
        for mut i in 0..2_u64.pow(index) {
            let mut running_total = *pieces.iter().nth(0).unwrap();

            for j in 0..index{
                let last_bit = i & 1;
                i = i >> 1;
                if  last_bit == 0 {
                    running_total = running_total + pieces.iter().nth(j as usize + 1).unwrap();
                }
                else {
                    running_total = running_total * pieces.iter().nth(j as usize + 1).unwrap();
                }
            }

            if running_total == total {
                good_eqns += 1;
                total_calibration += total;
                break;
            }
        }
        // Part 2
        let running_total = pieces.remove(0);
        if test_eqn(total, running_total, pieces) {
            good_eqns_2 += 1;
            total_calibration_2 += total;
        }


    }

    println!("There are {} good equations.", good_eqns);
    println!("The calibration result is {}.", total_calibration);

    println!("There are {} good equations Pt 2.", good_eqns_2);
    println!("The calibration result is {} Pt 2.", total_calibration_2);

    Ok(())
}