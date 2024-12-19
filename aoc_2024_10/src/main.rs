
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
    let mut score = 0;
    let mut rating = 0;
    for (i, row) in puzzle.iter().enumerate(){
        for (j,space) in row.iter().enumerate() {
            if *space == 0 {
                let mut positions     = HashSet::new();
                let mut routes = 1;
                (positions, routes) = get_hiking_score((i,j, 0), &puzzle, positions, routes);
                score += positions.iter().filter(|x| puzzle[x.0][x.1] == 9).count();
                rating += routes
            }
        }
    }
    println!("The total hiking score is {}", score);
    println!("The total hiking rating is {}", rating);
    Ok(())
}


fn get_hiking_score(start: (usize,usize,usize), puzzle: &Vec<Vec<u32>>, mut positions: HashSet<(usize,usize,usize)>,  mut routes: i32) -> (HashSet<(usize,usize,usize)>, i32) {
    positions.insert(start);
    let mut local_routes = 0;
    let height = *puzzle.get(start.0).unwrap().get(start.1).unwrap();
    //up
    if start.0 > 0 && puzzle[start.0-1][start.1] == height + 1 {
        local_routes += 1; 
        (positions, routes) = get_hiking_score((start.0-1,start.1, (height + 1) as usize), puzzle, positions, routes);
    }
    //left
    if start.1 > 0 && puzzle[start.0][start.1-1] == height + 1 {
        local_routes += 1;
         (positions, routes) = get_hiking_score((start.0,start.1-1, (height + 1) as usize), puzzle, positions, routes);
        }
    //down
    if start.0 < puzzle.len()-1 && puzzle[start.0+1][start.1] == height + 1 {
        local_routes += 1;
         (positions, routes) = get_hiking_score((start.0+1,start.1, (height + 1) as usize), puzzle, positions, routes);
        }
    //right
    if start.1 < puzzle[0].len()-1 && puzzle[start.0][start.1+1] == height + 1 {
        local_routes += 1;
         (positions, routes) = get_hiking_score((start.0,start.1+1, (height + 1) as usize), puzzle, positions, routes);
        }
    //routes += 1;
    if local_routes > 1 {routes += local_routes - 1}
    if local_routes == 0 && height < 9 {routes -= 1}

    return (positions, routes);
}