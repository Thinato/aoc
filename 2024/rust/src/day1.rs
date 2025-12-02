use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let file = File::open("inputs/day1").unwrap();
    let reader = BufReader::new(file);

    let mut password_part1 = 0;
    let mut password_part2 = 0;
    let mut left_values: Vec<i64> = Default::default();
    let mut right_values: Vec<i64> = Default::default();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split("   ").collect();
        let left = parts[0].parse::<i64>().unwrap();
        let right = parts[1].parse::<i64>().unwrap();

        left_values.push(left);
        right_values.push(right);
    }

    left_values.sort();
    right_values.sort();

    for i in 0..left_values.len() {
        let diff = left_values[i] - right_values[i];
        password_part1 += diff.abs()
    }

    let mut cache = HashMap::new();

    for l in 0..left_values.len() {
        let mut count = 0;
        match cache.get(&left_values[l]) {
            Some(cached) => password_part2 += cached,
            None => (),
        };

        for r in 0..right_values.len() {
            if right_values[r] > left_values[l] {
                break;
            }

            if right_values[r] == left_values[l] {
                right_values[r] = 0;
                count += 1;
            }
        }

        let result = left_values[l] * count;
        password_part2 += result;
        cache.entry(left_values[l]).or_insert(result);
    }

    println!("password part 1: {}", password_part1);
    println!("password part 2: {}", password_part2);
}
