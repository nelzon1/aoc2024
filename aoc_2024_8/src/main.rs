use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let result = line?;
         puzzle.push(result.chars().into_iter().collect::<Vec<char>>());
    }
    let _max_y = puzzle.len() as i32;
    let _max_x = puzzle[0].len() as i32;
    // Map the nodes
    let mut anti_node_map:HashSet<(i32,i32)> = HashSet::new();
    let mut node_map:HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    for (i,row) in puzzle.iter().enumerate() {
        for (j,space) in row.iter().enumerate() {
            if *space != '.' {
                node_map.entry(*space).or_insert(Vec::new()).push((i as i32, j as i32));
    }   }   }
    // Iterate over the map entries
    for (_node_key, positions) in node_map.iter() {
        // Each pair of nodes, calculate node positions and add to set
        for (i,  a) in positions.iter().enumerate() {
            for  b in positions.iter().skip(i + 1) {
                let (y,x): (i32, i32);
                y = a.0 - b.0;
                x = a.1 - b.1;
                // first anti-node
                if a.0 + y >= 0 &&  a.0 + y < _max_y && a.1 + x >= 0 && a.1 + x < _max_x {
                    anti_node_map.insert((a.0 + y, a.1 + x));
                }
                // second anti-node
                if b.0 - y >= 0 &&  b.0 - y < _max_y && b.1 - x >= 0 && b.1 - x < _max_x {
                    anti_node_map.insert((b.0 - y, b.1 - x));
                }
                let (mut s, mut t):((i32,i32),(i32,i32)) = (*a, *b);
                // part two
                anti_node_map.insert(*a);
                anti_node_map.insert(*b);
                while s.0 + y >= 0 &&  s.0 + y < _max_y && s.1 + x >= 0 && s.1 + x < _max_x {
                    anti_node_map.insert((s.0 + y, s.1 + x));
                    s.0 += y;
                    s.1 += x;
                }
                // second anti-node
                while t.0 - y >= 0 &&  t.0 - y < _max_y && t.1 - x >= 0 && t.1 - x < _max_x {
                    anti_node_map.insert((t.0 - y, t.1 - x));
                    t.0 -= y;
                    t.1 -= x;
                }
            }
        }
    }

    // return the count of anti-nodes
    println!("There are {} unique places where anti-nodes are located.", anti_node_map.iter().count());

    Ok(())
}
