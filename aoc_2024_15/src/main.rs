use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn move_robot(puzzle:&mut Vec<Vec<char>>, pos: &mut (usize,usize), dir: char) {
    // take direction and move in correct direction
    // while next space is O
    // look 1 index over
    // if wall, no movement
    // if space, move the robot and make the space a barrel
    match dir {
        '^' => {
            for i in 0..pos.1{
                let next_space = puzzle[pos.1-1-i][pos.0];
                if next_space == '#' { break;} //wall hit before any movement
                else if next_space == '.' { // open space, shift
                    if i > 0 {puzzle[pos.1-1-i][pos.0] = 'O'};
                    puzzle[pos.1-1][pos.0] = '@';
                    puzzle[pos.1][pos.0] = '.';
                    pos.1 = pos.1-1;
                    break;
                } // else it is a barrel, continue checking next in direction
            }
        }
        'v' => {
            for i in 0..puzzle.len() - pos.1{
                let next_space = puzzle[pos.1+1+i][pos.0];
                if next_space == '#' { break;} //wall hit before any movement
                else if next_space == '.' { // open space, shift
                    if i > 0 {puzzle[pos.1+1+i][pos.0] = 'O'};
                    puzzle[pos.1+1][pos.0] = '@';
                    puzzle[pos.1][pos.0] = '.';
                    pos.1 = pos.1+1;
                    break;
                } // else it is a barrel, continue checking next in direction
            }
        }
        '>' => {
            for i in 0..puzzle.len() - pos.0{
                let next_space = puzzle[pos.1][pos.0+1+i];
                if next_space == '#' { break;} //wall hit before any movement
                else if next_space == '.' { // open space, shift
                    if i > 0 {puzzle[pos.1][pos.0+1+i] = 'O'};
                    puzzle[pos.1][pos.0+1] = '@';
                    puzzle[pos.1][pos.0] = '.';
                    pos.0 = pos.0+1;
                    break;
                } // else it is a barrel, continue checking next in direction
            }
        }
        '<' => {
            for i in 0..pos.0{
                let next_space = puzzle[pos.1][pos.0-1-i];
                if next_space == '#' { break;} //wall hit before any movement
                else if next_space == '.' { // open space, shift
                    puzzle[pos.1][pos.0-1-i] = 'O';
                    puzzle[pos.1][pos.0-1] = '@';
                    puzzle[pos.1][pos.0] = '.';
                    pos.0 = pos.0-1;
                    break;
                } // else it is a barrel, continue checking next in direction
            }
        }
        _ => {

        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle:Vec<Vec<char>> = Vec::new();
    let mut moveset:Vec<char> = Vec::new();
    let mut first_part = true;
    for line in reader.lines() {
        let str = line?;
        if str.len() == 0 {
            first_part = false;
            continue;
        }
        if first_part {
            puzzle.push(str.chars().into_iter().collect::<Vec<char>>());
        }
        else{
            moveset.extend(str.chars().into_iter().collect::<Vec<char>>());
        }
    }
    // for move in moveset
    // move the robot
    let mut current_pos:(usize,usize) = (0,0);
    for (i,row) in puzzle.iter().enumerate() {
        if let Some(index) =  row.iter().position(|x| *x=='@'){
            current_pos = (i,index);
        }
    }
    print_puzzle(&puzzle);
    for movement in moveset {
        move_robot(&mut puzzle, &mut current_pos, movement);
        //print_puzzle(&puzzle);
    }
    print_puzzle(&puzzle);
    println!("The total gps score is {}", calculate_gps(&puzzle));
    Ok(())
}

fn print_puzzle(puzzle: &Vec<Vec<char>>) {
    println!();
    for row in puzzle {
        println!("{}",row.iter().collect::<String>());
    }
    println!();
}


fn calculate_gps(puzzle: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    for (i,row) in puzzle.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'O' {
                score += (i as u32) * 100  + (j as u32);
            }
        }
    }
    return score;
}