use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let file = File::open("inputs/day1").unwrap();
    let reader = BufReader::new(file);

    let mut current_dial: i32 = 50;
    let mut password_part1 = 0;
    let mut password_part2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let value;

        let is_right;
        if line.starts_with("R") {
            is_right = true;
            value = line[1..].parse::<i32>().unwrap();
        } else {
            is_right = false;
            value = line[1..].parse::<i32>().unwrap();
        }

        for _ in 0..value {
            if is_right {
                current_dial = (current_dial + 1).rem_euclid(100);
            } else {
                current_dial = (current_dial - 1).rem_euclid(100);
            }

            if current_dial == 0 {
                password_part2 += 1;
            }
        }
        if current_dial == 0 {
            password_part1 += 1;
        }
    }
    println!("password part 1: {}", password_part1);
    println!("password part 2: {}", password_part2);
}
