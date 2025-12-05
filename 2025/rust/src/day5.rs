use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let file = File::open("inputs/day5").unwrap();
    let reader = BufReader::new(file);

    let mut spoiled_count = 0;
    let mut fresh_count = 0;

    let mut ranges: Vec<(i64, i64)> = Vec::default();

    let mut reading_ranges = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            ranges = merge_ranges(ranges.clone());
            println!("ranges: {:?}", ranges);
            reading_ranges = false;
        } else if reading_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();
            ranges.push((start, end));
        } else {
            let value = line.parse::<i64>().unwrap();
            for (start, end) in &ranges.clone() {
                if value >= *start && value <= *end {
                    println!("{} is fresh", value);
                    spoiled_count += 1;
                }
            }
        }
    }

    for (start, end) in ranges.clone() {
        println!("fresh: {} - {} = {}", end, start, end - start);
        fresh_count += (end + 1) - start
    }

    println!("part 1: {}", spoiled_count);
    println!("part 2: {}", fresh_count);
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|&(start, _)| start);
    let mut merged_ranges = Vec::default();
    let mut current_range = ranges[0];

    for range in ranges {
        if range.0 <= current_range.1 + 1 {
            current_range.1 = current_range.1.max(range.1);
        } else {
            merged_ranges.push(current_range);
            current_range = range;
        }
    }
    merged_ranges.push(current_range);
    merged_ranges
}
