use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::thread;
use std::time::Duration;

pub fn execute() {
    let file = File::open("inputs/day4").unwrap();
    let reader = BufReader::new(file);

    let mut map = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        map.push(line);
    }

    let rows = map.len();
    let cols = map[0].chars().count();

    let mut accessible = 0;
    let mut last_round = 1000;
    let mut round_counter = 1;

    while last_round > 0 {
        last_round = 0;
        let mut new_map = Vec::new();
        print!("\x1B[2J\x1B[1;1H");
        println!("Round {}! =====", round_counter);
        println!("rows: {}, cols: {}", map.len(), map[0].chars().count());
        for row in 0..rows {
            let mut new_line = String::new();
            for col in 0..cols {
                if map[row].chars().nth(col).unwrap() != '@' {
                    new_line.push('.');
                    print!(".");
                    continue;
                }
                let mut adjacent_rolls = 0;
                if row != 0 {
                    if col != 0 {
                        adjacent_rolls += check_pos(map.clone(), row - 1, col - 1);
                    }
                    adjacent_rolls += check_pos(map.clone(), row - 1, col);
                    if col != cols - 1 {
                        adjacent_rolls += check_pos(map.clone(), row - 1, col + 1);
                    }
                }
                if col != 0 {
                    adjacent_rolls += check_pos(map.clone(), row, col - 1);
                }
                if col != cols - 1 {
                    adjacent_rolls += check_pos(map.clone(), row, col + 1);
                }
                if row != rows - 1 {
                    if col != 0 {
                        adjacent_rolls += check_pos(map.clone(), row + 1, col - 1);
                    }
                    adjacent_rolls += check_pos(map.clone(), row + 1, col);
                    if col != cols - 1 {
                        adjacent_rolls += check_pos(map.clone(), row + 1, col + 1);
                    }
                }

                if adjacent_rolls < 4 {
                    last_round += 1;
                    new_line.push('.');
                    print!("x")
                } else {
                    new_line.push('@');
                    print!("@")
                }
            }
            print!("\n");
            new_map.push(new_line);
        }
        map = new_map;
        accessible += last_round;
        println!("===== {} rolls of paper removed", last_round);
        round_counter += 1;
        thread::sleep(Duration::from_millis(100));
    }
    println!("\n\nresult {}", accessible)
}

fn check_pos(map: Vec<String>, x: usize, y: usize) -> u8 {
    if map[x].chars().nth(y).unwrap() == '@' {
        return 1;
    }

    return 0;
}
