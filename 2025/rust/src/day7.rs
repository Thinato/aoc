use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let path = "inputs/day7";
    // let part1 = part1(path);
    let part2 = part2(path);
    // println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn part2(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut beams: Vec<usize> = vec![];
    let mut quantum_beams: Vec<usize> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let chars = line.chars();
        let mut pline = String::new();
        if quantum_beams.len() == 0 {
            quantum_beams = vec![0; chars.clone().count()];
        }

        let mut new_q_beams_sum = vec![0; chars.clone().count()];
        let mut keep_q_beams = quantum_beams.clone();
        let mut should_sum = false;
        for j in 0..chars.clone().count() {
            let c = chars.clone().nth(j).unwrap();
            pline.push(c);
            match c {
                '.' => continue,
                'S' => {
                    beams = vec![j];
                    quantum_beams[j] = 1;
                    new_q_beams_sum[j] = 1
                }
                '^' => {
                    let mut new_beams: Vec<usize> = vec![];
                    let mut new_q_beams = vec![0; chars.clone().count()];
                    // println!("bm {:?}", beams);
                    for beam in 0..beams.len() {
                        if beams[beam] == j {
                            new_beams.push(j - 1);
                            new_beams.push(j + 1);
                            new_q_beams[j - 1] += quantum_beams[j];
                            new_q_beams[j + 1] += quantum_beams[j];
                            keep_q_beams[j] = 0;
                        } else {
                            new_beams.push(beams[beam]);
                        }
                    }
                    for i in 0..new_q_beams.len() {
                        new_q_beams_sum[i] += new_q_beams[i];
                        should_sum = true
                    }
                    println!("nq {:?}", new_q_beams);
                    new_beams.dedup();
                    beams = new_beams.clone();
                }
                _ => println!("unhandled char: {}", c),
            }
        }
        for i in 0..keep_q_beams.len() {
            new_q_beams_sum[i] += keep_q_beams[i];
            should_sum = true
        }
        if should_sum {
            quantum_beams = new_q_beams_sum.clone();
        }
        println!("kq {:?}", keep_q_beams);
        println!(" q {:?}", quantum_beams);

        let mut chars: Vec<char> = pline.chars().collect();

        for &beam in &beams {
            chars[beam] = '|';
        }

        println!("{}", chars.into_iter().collect::<String>())
    }
    quantum_beams.iter().sum::<usize>() as i64
}

fn part1(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut beams: Vec<usize> = vec![];
    let mut quantum_beams: Vec<usize> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let chars = line.chars();
        let mut pline = String::new();
        if quantum_beams.len() == 0 {
            for _ in 0..chars.clone().count() {
                quantum_beams.push(0)
            }
        }
        if quantum_beams.len() > 3292 {
            break;
        }

        for j in 0..chars.clone().count() {
            let c = chars.clone().nth(j).unwrap();
            pline.push(c);
            match c {
                '.' => continue,
                'S' => beams = vec![j],
                '^' => {
                    let mut new_beams: Vec<usize> = vec![];
                    for beam in 0..beams.len() {
                        if beams[beam] == j {
                            part1 += 1;
                            new_beams.push(j - 1);
                            new_beams.push(j + 1);
                        } else {
                            new_beams.push(beams[beam]);
                        }
                    }
                    new_beams.dedup();
                    beams = new_beams.clone();
                }
                _ => println!("unhandled char: {}", c),
            }
        }
        println!("{:?}", quantum_beams);

        let mut chars: Vec<char> = pline.chars().collect();

        for &beam in &beams {
            chars[beam] = '|';
        }

        println!("{}", chars.into_iter().collect::<String>())
    }

    part1
}
