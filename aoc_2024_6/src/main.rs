use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn part_one_mouse(mut puzzle: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut dir:i32 = 0; // (0 ,1 , 2, 3)  % 4 // 0 = up, 1 = r, 2 = d, 3 = l
    let mut x:usize;
    let mut y:usize;
    (x,y) = find_start(&puzzle);
    loop {
        //cover our current tile with an X
        puzzle[y][x] = 'X';
        //take a step, else turn right
        let temp_x:i32;
        let temp_y:i32;
        (temp_x, temp_y, dir) = get_next_space(&puzzle, x, y, dir);
        if temp_x < 0 || temp_x >= puzzle[0].len() as i32 || temp_y < 0 || temp_y >= puzzle.len() as i32{
            return puzzle;
        }
        x = temp_x as usize;
        y = temp_y as usize; 
        // println!("{}", "- - - - - -");
        // for row in &puzzle{
        //     println!("{}", row.iter().collect::<String>());
        // }

    }
}

fn get_next_space(puzzle: &Vec<Vec<char>>, x:usize, y:usize, mut dir:i32) -> (i32,i32,i32) {
    let mut temp_x:i32 = 0;
    let mut temp_y:i32 = 0;
    let mut next_space:char = '#';

    while next_space == '#' {
        match dir  {
            0 => {
                temp_x = x as i32;
                temp_y = y as i32 - 1;
            }
            1 => {
                temp_x = x as i32 + 1;
                temp_y = y as i32;
            }
            2 => {
                temp_x = x as i32;
                temp_y = y as i32 + 1;
            }
            3 => {
                temp_x = x as i32 - 1;
                temp_y = y as i32;
            }
            _ => {
                return (-1,-1,-1);
            }
        }

        if temp_x < 0 || temp_x >= puzzle[0].len() as i32 || temp_y < 0 || temp_y >= puzzle.len() as i32{
            return (temp_x,temp_y, dir);
        }

        next_space = puzzle[temp_y as usize][temp_x as usize];
        if next_space == '#'{
            dir = (dir + 1) % 4;
        }
            
    }
    
    return (temp_x, temp_y, dir);
}

fn find_start(puzzle: &Vec<Vec<char>>) -> (usize, usize){
    for i in 0..puzzle.len() {
        for j in 0..puzzle.len() {
            if puzzle[i][j] == '^' {
                return (j,i);
            }
        }
    }
    return (0,0);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let result = line?;
         puzzle.push(result.chars().collect());
    }

    puzzle = part_one_mouse(puzzle);

    println!("There are {} tiles covered in part one.", puzzle.iter().map(|x| x.iter().filter(|y| **y == 'X').count() ).sum::<usize>() );
    println!("{}", "- - - - - -");
    for row in &puzzle{
        println!("{}", row.iter().collect::<String>());
    }

    Ok(())
}
