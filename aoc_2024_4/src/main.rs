use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let mut puzzle:Vec<Vec<char>> = Vec::new();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let _ = puzzle.push(line?.chars().collect::<Vec<char>>());
    }

    println!("The count of XMAS in the puzzle is: {}.", count_xmas(&puzzle));
    println!("The count of X-MAS in the puzzle is: {}.", count_x_mas(&puzzle));
    Ok(())
}

fn count_x_mas(matrix:&Vec<Vec<char>> ) -> usize {
    let mut count:usize = 0;
    for i in 1..matrix.len()-1{
        for j in 1..matrix[i].len()-1{
            if matrix[i][j] != 'A' {
                continue;
            }
            if (matrix[i+1][j+1] == 'S' && matrix[i-1][j-1] == 'M' || matrix[i+1][j+1] == 'M' && matrix[i-1][j-1] == 'S')
            && (matrix[i+1][j-1] == 'S' && matrix[i-1][j+1] == 'M' || matrix[i+1][j-1] == 'M' && matrix[i-1][j+1] == 'S') {
                count += 1;
            }
        }
    }
    return count;
}



 fn count_xmas(matrix:&Vec<Vec<char>> ) -> usize {
    let mut count:usize = 0;
    let xmas:String = "XMAS".to_string();

    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            if matrix[i][j] != 'X' {
                continue;
            }
            let mut test_string:Vec<char> = Vec::new();
            let mut test_strings:Vec<String> = Vec::new();
            //Right
            if let Some(test_vec) = matrix[i].get(j..j+4){
                    test_strings.push( test_vec.iter().collect::<String>());
            }
            //Left
            if j > 2 {
                if let Some(test_vec) = matrix[i].get(j-3..j+1){
                    test_strings.push( test_vec.iter().rev().collect::<String>());
                }
            }
            // Up
            if i > 2 {
                    test_strings.push( matrix.get(i-3..i+1).unwrap().iter().rev().map(|row | row.get(j).unwrap()).collect::<String>() );
            }
            
            // Down
            if let Some(test_vec) = matrix.get(i..i+4) {
                    test_strings.push(test_vec.iter().map(|x| x.get(j).unwrap()).collect::<String>() );
            }
            
            // Up + Right
            if i > 2 {
                    for (k, line) in matrix.get(i-3..i+1).unwrap().iter().rev().enumerate() {
                        if let Some(new_char) = line.get(j+k){
                            test_string.push(*new_char);
                        }
                        
                    }
                    test_strings.push(test_string.iter().collect::<String>());
                    test_string.clear();
            }
            // Down + Right
            if let Some(chunk) = matrix.get(i..i+4) {
                for (k, line) in chunk.iter().enumerate() {
                    if let Some(new_char) = line.get(j+k){
                        test_string.push(*new_char);
                    }
                    
                }
            }
            test_strings.push(test_string.iter().collect::<String>());
            test_string.clear();

            // Up + Left
            if i > 2 && j > 2{
                    for (k, line) in matrix.get(i-3..i+1).unwrap().iter().rev().enumerate() {
                        if let Some(new_char) = line.get(j-k){
                            test_string.push(*new_char);
                        }
                        
                    }
                    test_strings.push(test_string.iter().collect::<String>());
                    test_string.clear();
            }

            // Down + Left
            if j > 2 {
                if let Some(chunk) = matrix.get(i..i+4) {
                    for (k, line) in chunk.iter().enumerate() {
                        if let Some(new_char) = line.get(j-k){
                            test_string.push(*new_char);
                        }
                        
                    }
                    test_strings.push(test_string.iter().collect::<String>());
                    test_string.clear();
                }
            }

            // Check test strings
            count += test_strings.iter().filter( |&x| x.eq(&xmas) ).count();
        }
    }
    return count;
}