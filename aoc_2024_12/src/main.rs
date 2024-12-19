use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn fence_region(x:usize, y:usize, puzzle: &mut Vec<Vec<char>>, start_character: char, positions: &mut HashSet<(usize,usize)>) -> (i32,i32) {
    //if puzzle[x][y] == '░' || puzzle[x][y] == '▒'  || puzzle[x][y] ==  '▓' {
    let mut area = 0;
    let mut perimeter = 0;
    if puzzle[y][x] == '░'{
        return (area, perimeter);
    }
    if puzzle[y][x] != start_character {
        perimeter += 1;
        return (area, perimeter);
    }
    area += 1;
    puzzle[y][x] = '░';
    positions.insert((x,y));

    //up
    if y>0 {
        let neighbour = fence_region(x, y-1, puzzle, start_character, positions);
        area += neighbour.0;
        perimeter += neighbour.1;
    }
    else {
        perimeter += 1;
    }
    //down
    if y<puzzle.len()-1 {
        let neighbour = fence_region(x, y+1, puzzle, start_character, positions);
        area += neighbour.0;
        perimeter += neighbour.1;
    }
    else {
        perimeter += 1;
    }
    //right
    if x<puzzle[0].len()-1 {
        let neighbour = fence_region(x+1, y, puzzle, start_character, positions);
        area += neighbour.0;
        perimeter += neighbour.1;
    }
    else {
        perimeter += 1;
    }
    //left
    if x>0 {
        let neighbour = fence_region(x-1, y, puzzle, start_character, positions);
        area += neighbour.0;
        perimeter += neighbour.1;
    }
    else {
        perimeter += 1;
    }

    return (area,perimeter);
}

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle:Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        puzzle.push(line?.chars().into_iter().collect::<Vec<char>>());
    }
    let mut fence_cost = 0;
    let mut discount = 0;
    for y in 0..puzzle.len() {
        for x in 0..puzzle[y].len() {
            if puzzle[y][x] != '▓' {
                let mut positions:HashSet<(usize,usize)> = HashSet::new();
                let start_character = puzzle[y][x];
                let params = fence_region(x, y, &mut puzzle, start_character, &mut positions);
                fence_cost += params.0 * params.1;
                println!("");
                for row in &puzzle{
                    println!("{}",row.iter().collect::<String>());
                }
                let corners = positions.iter().map(|(i,j)| count_corners(*i, *j, &puzzle)).sum::<u32>();
                discount += params.0 * corners as i32;
                println!("Area: {} Perimeter: {} Corners: {} Price: {}", params.0, params.1,corners, corners as i32 * params.0 );
                //reset map
                for (j,i) in positions{
                    puzzle[i][j] = '▓';
                }
            }
        }
    }
    println!("The total fence cost is ${}", fence_cost);
    println!("The discounted fence cost is ${}", discount);
    // for row in &puzzle{
    //     println!("{}",row.iter().collect::<String>());
    // }
    Ok(())
}

fn count_corners(x:usize, y:usize, puzzle: &Vec<Vec<char>>) -> u32 {
    let this_char = puzzle[y][x];
    let mut corners:u32 = 0;

    // UR
    if (y>0 && x<puzzle[y].len()-1 && ((puzzle[y][x+1] != this_char && puzzle[y-1][x] != this_char) 
    || ( puzzle[y][x+1] == this_char && puzzle[y-1][x] == this_char && puzzle[y-1][x+1] != this_char)))
    || (y>0 && x == puzzle[y].len()-1 && puzzle[y-1][x] != this_char)
    || (y==0 && x < puzzle[y].len()-1 && puzzle[y][x+1] != this_char)
    || (y==0 && x == puzzle[y].len()-1) {
        corners+=1;
    }
    // UL
    if (y>0 && x>0 && ((puzzle[y][x-1] != this_char && puzzle[y-1][x] != this_char) 
    || ( puzzle[y][x-1] == this_char && puzzle[y-1][x] == this_char && puzzle[y-1][x-1] != this_char)))
    || (y>0 && x == 0 && puzzle[y-1][x] != this_char)
    || (y==0 && x > 0 && puzzle[y][x-1] != this_char)
    || (y==0 && x == 0) {
        corners+=1;
    }

    // DR
    if (y<puzzle.len()-1 && x<puzzle[y].len()-1 && ((puzzle[y][x+1] != this_char && puzzle[y+1][x] != this_char) 
    || ( puzzle[y][x+1] == this_char && puzzle[y+1][x] == this_char && puzzle[y+1][x+1] != this_char)))
    || (y<puzzle.len()-1 && x == puzzle[y].len()-1 && puzzle[y+1][x] != this_char)
    || (y==puzzle.len()-1 && x < puzzle[y].len()-1 && puzzle[y][x+1] != this_char)
    || (y==puzzle.len()-1 && x == puzzle[y].len()-1) {
        corners+=1;
    }

    // DL
    if (y<puzzle.len()-1 && x>0 && ((puzzle[y][x-1] != this_char && puzzle[y+1][x] != this_char) 
    || ( puzzle[y][x-1] == this_char && puzzle[y+1][x] == this_char && puzzle[y+1][x-1] != this_char)))
    || (y<puzzle.len()-1 && x == 0 && puzzle[y+1][x] != this_char)
    || (y==puzzle.len()-1 && x > 0 && puzzle[y][x-1] != this_char)
    || (y==puzzle.len()-1 && x == 0) {
        corners+=1;
    }

    return corners;
}