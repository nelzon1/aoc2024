use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use regex::Regex;


fn is_report_sorted( report:&Vec<&str>, rules:&Vec<Vec<&str>>) -> i32 {
    for i in 0..report.len()-1 {
        for j in i+1.. report.len() {
            if !compare(report[i].parse::<i32>().unwrap(), report[j].parse::<i32>().unwrap(), rules) {
                return 0;
            }
        }
    }
    return report[report.len()/2].parse::<i32>().unwrap();
}

fn is_report_sorted_part_two(mut report: Vec<&str>, rules:&Vec<Vec<&str>>) -> i32 {
    if is_report_sorted(&report, rules) > 0 {
        return 0;
    }
    while is_report_sorted(&report, rules) == 0 {
        for i in 0..report.len()-1 {
            for j in i+1.. report.len() {
                if !compare_part_two(report[i].parse::<i32>().unwrap(), report[j].parse::<i32>().unwrap(), rules) {
                    let tmp = report[i];
                    report[i] = report[j];
                    report[j] = tmp;
                }
            }
        }
    }
    return report[report.len()/2].parse::<i32>().unwrap();
}

fn compare(a:i32, b:i32, rules:&Vec<Vec<&str>>) -> bool {
    for rule in rules {
        if rule[0].parse::<i32>().unwrap() == b && rule[1].parse::<i32>().unwrap() == a {
        return false;
        }
    }
    return true;
}

fn compare_part_two(a:i32, b:i32, rules:&Vec<Vec<&str>>) -> bool {
    for rule in rules {
        if rule[0].parse::<i32>().unwrap() == b && rule[1].parse::<i32>().unwrap() == a {
        return false;
        }
    }
    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    const DEBUG:bool = false;
    let file_path_rule = if DEBUG {"debug_rule.txt"} else {"input_rule.txt"};
    let file_path_report = if DEBUG {"debug_report.txt"} else {"input_report.txt"};
    let file_rule = File::open(file_path_rule)?;
    let regex_pattern = r"(\d+)\|(\d+)";
    let regex = Regex::new(regex_pattern)?;


    let mut reader = io::BufReader::new(file_rule);
    let mut input = String::new() ;
    let _ = reader.read_to_string(&mut input);
    let rules = regex.captures_iter( input.split("\n\n").collect::<Vec<&str>>()[0] ).map(|cap| cap.iter().skip(1).flat_map(|x| x.map(|y| y.as_str())).collect::<Vec<&str>>() ).collect::<Vec<Vec<&str>>>();

    let input = std::fs::read_to_string(file_path_report).expect("Failed to read file");
    let mut reports: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let result: Vec<&str> = line.split(',').collect();
        reports.push(result);
    }

    println!("There are {} sorted reports", reports.iter().filter(|x| is_report_sorted(x, &rules) > 0 ).count() );
    println!("The sum is {}", reports.iter().map(|x| is_report_sorted(x, &rules) ).sum::<i32>() );
    println!("There are {} sorted reports", reports.iter().filter(|x| is_report_sorted_part_two(  x.to_vec(), &rules) > 0 ).count() );
    println!("The sum is {}", reports.iter().map(|x| is_report_sorted_part_two( x.to_vec(), &rules) ).sum::<i32>() );

    Ok(())
}
