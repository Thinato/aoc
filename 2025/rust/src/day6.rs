use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let file = File::open("inputs/day6").unwrap();
    let reader = BufReader::new(file);

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut values: Vec<Vec<i64>> = vec![];
    let mut all_lines: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        all_lines.push(line.clone());
        let numbers: Vec<_> = line.split(" ").collect();
        let filtered: Vec<&&str> = numbers.iter().filter(|x| **x != "").collect();

        for i in 0..filtered.len() {
            let element = filtered[i];
            if *element == "+" {
                match values.get(i) {
                    Some(_) => {
                        part_1 += values[i].iter().copied().reduce(|acc, e| acc + e).unwrap();
                    }
                    None => println!("shit"),
                }
            } else if *element == "*" {
                match values.get(i) {
                    Some(_) => {
                        part_1 += values[i].iter().copied().reduce(|acc, e| acc * e).unwrap();
                    }
                    None => println!("shit"),
                }
            } else {
                match values.get(i) {
                    Some(_) => {
                        values[i].push(element.parse::<i64>().unwrap());
                    }
                    None => {
                        values.push(vec![element.parse::<i64>().unwrap()]);
                    }
                }
            }
        }
    }

    let mut value_acc: Vec<i64> = vec![];

    for i in (0..all_lines[0].len()).rev() {
        let mut current_acc = String::new();
        for j in 0..all_lines.len() {
            match all_lines[j].chars().nth(i) {
                Some(char) => current_acc.push(char),
                None => (),
            };
        }
        println!("line: {:?}", current_acc);
        let last_char = current_acc.trim().chars().last().unwrap_or('?');

        if last_char == '+' {
            let value = current_acc[0..current_acc.len()-1].trim().parse::<i64>().unwrap();
            value_acc.push(value);
            println!("sum: {:?}", value_acc);

            part_2 += value_acc.iter().copied().reduce(|acc, e| acc + e).unwrap();
            value_acc = vec![];
        } else if last_char == '*' {
            let value = current_acc[0..current_acc.len()-1].trim().parse::<i64>().unwrap();
            value_acc.push(value);
            println!("mul: {:?}", value_acc);

            part_2 += value_acc.iter().copied().reduce(|acc, e| acc * e).unwrap();
            value_acc = vec![];
        } else if current_acc.trim() != "" {
            let value = current_acc.trim().parse::<i64>().unwrap();
            value_acc.push(value);
        }
    }

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}
