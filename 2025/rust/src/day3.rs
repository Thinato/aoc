use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let file = File::open("inputs/day3").unwrap();
    let reader = BufReader::new(file);

    let mut joltage_part1 = 0;
    let mut joltage_part2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        joltage_part1 += part1(line.clone());
        joltage_part2 += part2(line);
    }
    println!("password part 1: {}", joltage_part1);
    println!("password part 2: {}", joltage_part2);
}

fn part1(line: String) -> u32 {
    let mut left = 0;
    let mut left_pos = 0;
    for i in 0..line.len() {
        let value = line.chars().collect::<Vec<_>>()[i].to_digit(10).unwrap();
        if i == line.len() - 1 {
            break;
        }
        if value > left {
            left = value;
            left_pos = i
        }
    }

    let mut right = 0;
    for i in left_pos + 1..line.len() {
        let value = line.chars().collect::<Vec<_>>()[i].to_digit(10).unwrap();
        if value > right {
            right = value
        }
    }

    (left * 10) + right
}

fn part2(line: String) -> i64 {
    let digits = 12;

    let mut str_result = String::from("");
    let mut next_j_pos = 0;
    for i in 0..digits {
        let mut current_digit = 0;

        for j in next_j_pos..(line.len()+1) - digits + i {
            let value = line.chars().collect::<Vec<_>>()[j].to_digit(10).unwrap();
            if value > current_digit {
                next_j_pos = j + 1;
                current_digit = value
            }
        }
        str_result.push_str(&current_digit.to_string());
    }


    str_result.parse::<i64>().unwrap()
}
