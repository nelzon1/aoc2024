use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let (x_len, y_len) = if DEBUG {(11,7)} else {(101,103)};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut robots:Vec<Vec<(i32,i32)>> = Vec::new();
    for line in reader.lines() {
        let string = line?;
        robots.push(string.split_whitespace()
        .into_iter()
        .map(|x| {
            if let (Some(first), Some(second)) = (x[2..].split(',').into_iter().nth(0), x[2..].split(',').into_iter().nth(1)) {
                (first.parse::<i32>().unwrap(),second.parse::<i32>().unwrap())
            }
            else{
                (-1,-1)
            }
        })
        .collect::<Vec<(i32,i32)>>());
    }
    let moves= 100;
    // for robot in &mut robots{
    //     move_robot(robot, moves, x_len, y_len);
    // }
    println!("The total safety score after {} seconds is {}.",moves,safety_score(&robots, x_len, y_len));
    let mut count =0;
    let mut lowest_var = 99999999;
    loop {
        count += 1;
        for robot in &mut robots{
            move_robot(robot, 1, x_len, y_len);
        }   
        let (var_x,var_y) = variance(&robots);
        if var_x + var_y < lowest_var {
            print_display(&robots, x_len, y_len);
            println!("Count: {} Variance: ({},{})", count, var_x, var_y);
            lowest_var = var_x + var_y
        }
        //sleep(Duration::from_millis(400));
    }
    Ok(())
}

fn move_robot(robot:&mut Vec<(i32,i32)>, moves: i32, x_len: i32, y_len: i32) {
    // x
    robot[0].0 = (robot[0].0 + robot[1].0 * moves).rem_euclid(x_len);
    // y
    robot[0].1 = (robot[0].1 + robot[1].1 * moves).rem_euclid(y_len);
}

fn safety_score(robots: &Vec<Vec<(i32,i32)>>, x_len:i32, y_len:i32) -> u64 {
    let (x_half, y_half) = (x_len / 2, y_len / 2);
    let (mut q1, mut q2, mut q3, mut q4) = (0,0,0,0);
    for robot in robots {
        if robot[0].0 < x_half && robot[0].1 < y_half {
            q1 += 1;
        }
        else if robot[0].0 < x_half && robot[0].1 > y_half {
            q2 += 1;
        }
        else if robot[0].0 > x_half && robot[0].1 > y_half {
            q3 += 1;
        }
        else if robot[0].0 > x_half && robot[0].1 < y_half {
            q4 += 1;
        }
    }
    return q1 * q2 * q3 * q4;
}

fn print_display(robots: &Vec<Vec<(i32,i32)>>, x_len:i32, y_len:i32) {
    let mut puzzle:Vec<Vec<char>> = Vec::new();
    for _i in 0..y_len {
        let mut row = Vec::new();
        for _j in 0..x_len{
            row.push(' ');
        }
        puzzle.push(row);
    }
    for robot in robots{
        puzzle[robot[0].1 as usize][robot[0].0 as usize] = '»';
    }
    println!("");
    println!("{}", "-".repeat( x_len as usize));
    for row in puzzle{
        println!("|{}|", row.iter().collect::<String>());
    }
    
}

fn variance(robots: &Vec<Vec<(i32,i32)>>) -> (u64, u64) {
    let (mut avg_x, mut avg_y) = (0,0); 
    for robot in robots {
        avg_x += robot[0].0;
        avg_y += robot[0].1;
    }
    avg_x = avg_x / robots.len() as i32;
    avg_y = avg_y / robots.len() as i32;
    let (mut var_x, mut var_y) = (0,0);
    for robot in robots {
        var_x += (robot[0].0 - avg_x).pow( 2) as u64;
        var_y += (robot[0].1 - avg_y).pow(2) as u64;
    }
    return (var_x, var_y);
}