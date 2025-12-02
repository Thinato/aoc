use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let file = File::open("inputs/day2").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer: String = Default::default();
    let mut sum1 = 0;
    let mut sum2 = 0;

    reader.read_line(&mut buffer).unwrap();

    for range in buffer.replace("\n", "").split(",") {
        let parts: Vec<_> = range.split("-").collect();
        let min = parts[0].parse::<i64>().unwrap();
        let max = parts[1].parse::<i64>().unwrap();

        for i in min..max + 1 {
            if part1(i.to_string()) {
                sum1 += i;
            }
            if part2(i.to_string()) {
                sum2 += i;
            }
        }
    }

    println!("result part1: {}", sum1);
    println!("result part2: {}", sum2);
}

fn part1(text: String) -> bool {
    let len = text.len();
    if len % 2 != 0 {
        return false;
    }
    if &text[0..len / 2] == &text[len / 2..len] {
        return true;
    }
    return false;
}

fn part2(text: String) -> bool {
    let len = text.len();

    if len < 2 {
        return false
    }

    if text
        .chars()
        .all(|c| c == text.chars().collect::<Vec<_>>()[0])
    {
        return true;
    }

    // if len % 2 != 0 {
    //     return false;
    // }

    for i in 1..len - 1 {
        if i + 1 > len / 2 {
            return false;
        }
        let start = &text[0..i + 1];
        // println!("start {}", start);
        if len % start.len() != 0 {
            continue;
        }

        let mut count = 0;
        for j in (0..len - 1).step_by(start.len()) {
            // println!("j {}, start.len {}", j, j+start.len());
            if j + start.len() > len  {
                break;
            }
            let current = &text[j..j + start.len()];
            // println!("current {}", current);

            if current != start {
                break;
            }
            count += 1;
            // println!("count {}", count);
        }

        // println!("len {}, start.len {}", len, start.len());
        if count == len / start.len() {
            return true;
        }
        // for j in (i + 1..len - 1).step_by(start.len()) {
        //     if j + start.len() > len {
        //         return false;
        //     }
        //     // println!("j {}, i {}, len {}, start.len {}", j, i, len, start.len());
        //     let current = &text[j..j + start.len()];
        //     if start != current {
        //         break;
        //     }
        // }
    }
    return false;
}
