use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::stdout;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path = if DEBUG {"debug2.txt"} else {"input.txt"};
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut puzzle:String = String::new();
    for line in reader.lines() {
        puzzle = line?;
    }
    println!("{}", puzzle);
    let mut disk_vec:Vec<u32> = Vec::new();
    let mut disk_pages:Vec<(u32,u32)> = Vec::new();
    // iterate over the puzzle and build the disk vector
    for (i, a) in puzzle.chars().into_iter().enumerate() {
        let file_len = a.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            for _ in 0..file_len {
                disk_vec.push((i / 2 ) as u32);
                
            }
            disk_pages.push(((i/2) as u32, file_len as u32));
        }
        else {
            for _ in 0..file_len {
                disk_vec.push(999999999);
            }
            disk_pages.push((999999999, file_len as u32));
        }
    }
    // literate until the disk string is compacted
    while let Some(next_gap) = disk_vec.iter().position(|x| *x == 999999999) {
        if next_gap >  disk_vec.len() - 2 {break;}
        let last_block = &disk_vec.pop().unwrap();
        disk_vec[next_gap] = *last_block;
    }
    // Part 2
    let mut index = disk_pages.len()-1;
    // literate until the disk string is compacted
    //print_disk(&disk_pages);
    while index > 0 {
        let mut next_file = disk_pages.iter().nth(index).unwrap();
        if let Some(next_block_to_move) = is_space_available(&disk_pages, next_file.1, index - 1) {
            next_file = disk_pages.iter().nth(index).unwrap();
            if next_file.0 == 999999999 {
                index -= 1;
                continue;
            }
            disk_pages = compress(*next_file, next_block_to_move, disk_pages);
            //print_disk(&disk_pages);
        }
        else {
            index -= 1;
        }
    }
    // get checksum
    println!("The checksum is {}", checksum(disk_vec));
    println!("The checksum is {}", checksum2(disk_pages));
    Ok(())
}

fn is_space_available(disk_pages:&Vec<(u32,u32)>, file_size:u32, index:usize) -> Option<usize> {
    return disk_pages.iter().take(index+1).position(|x| x.1 >= file_size && x.0 == 999999999);
}

fn compress(file:(u32,u32), index:usize, mut disk_pages:Vec<(u32,u32)>) -> Vec<(u32,u32)> {
    if disk_pages[index].0 != 999999999 {panic!();}
    // more space than needed
    if file.1 < disk_pages[index].1 {
        disk_pages[index].1 -= file.1;
        let mut slice = Vec::new();
        slice.push(file);
        let old_position = disk_pages.iter().position(|x: &(u32, u32)| *x == file).unwrap();
        disk_pages[old_position].0 = 999999999;
        disk_pages.splice(index..index, slice);
        return disk_pages;
    }
    // perfect fit
    else{
        let old_position = disk_pages.iter().position(|x: &(u32, u32)| *x == file).unwrap();
        disk_pages[old_position].0 = 999999999;
        disk_pages[index] = file;
        return disk_pages;
    }

}

fn checksum(input: Vec<u32>) -> u64 {
    return input.iter()
                .filter(|x| **x != 999999999)
                .enumerate()
                .map(|(i, x)| (i as u64) * *x as u64)
                .sum();
}

fn checksum2(input: Vec<(u32,u32)>) -> u64 {
    let mut index: u64 = 0;
    let mut total: u64 = 0;
    for page in input.iter(){
        if page.0 != 999999999{
            for _ in 0..page.1 {
                total += index * page.0 as u64;
                index += 1;
            }
    
        } 
        else{
            index += page.1 as u64;
        }
    }
    return total;
}

fn print_disk(input:& Vec<(u32,u32)>) {
    println!();
    for page in input{
        print!("{}","|");
        for _ in 0..page.1 {
            if page.0 == 999999999 {
                print!("{}",".");
                _ = stdout().flush();
            }
            else{
                print!("{}",page.0);
                _ =stdout().flush();
            }
        }
    }
    print!("{}","|");
    println!();
}