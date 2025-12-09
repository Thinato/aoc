use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn execute() {
    let mut part1 = 1;
    let mut part2 = 0;

    println!("getting boxes...");
    let boxes = get_boxes("inputs/day8-test");
    let mut circuits: Vec<Vec<usize>> = vec![];
    println!("getting box distances...");
    let box_distances = get_box_distances(boxes.clone());
    // println!("{:?}", box_distances);

    let mut connection_count = 0;
    let connection_limit = 10;
    for bx in box_distances {
        if connection_count >= connection_limit {
            break;
        }
        let mut already_in_circ = false;
        for circ in &circuits {
            if circ.contains(&bx.0) && circ.contains(&bx.1) {
                println!(
                    "ALR {:?} AND {:?}",
                    &boxes[bx.1], &boxes[bx.0]
                );
                already_in_circ = true;
                break;
            }
        }

        if already_in_circ {
            continue;
        }

        let mut a_in_circ = (false, 0);
        let mut b_in_circ = (false, 0);

        for i in 0..circuits.len() {
            if circuits[i].contains(&bx.0) {
                a_in_circ = (true, i);
                break;
            }
        }
        for i in 0..circuits.len() {
            if circuits[i].contains(&bx.1) {
                b_in_circ = (true, i);
                break;
            }
        }
        // println!("{}, {}, {}", bx.0, bx.1, bx.2);

        if a_in_circ.0 && !b_in_circ.0 {
            println!(
                "ADD {:?} TO {:?}'s circuit",
                &boxes[bx.1], &boxes[bx.0]
            );
            // println!(
            //     "ADD {:?} TO {:?}'s circuit",
            //     bx.1, bx.0
            // );
            circuits[a_in_circ.1].push(bx.1);
            connection_count += 1;
        } else if !a_in_circ.0 && b_in_circ.0 {
            println!(
                "ADD {:?} TO {:?}'s circuit",
                &boxes[bx.0], &boxes[bx.1]
            );
            // println!(
            //     "ADD {:?} TO {:?}'s circuit",
            //     bx.0, bx.1
            // );
            circuits[b_in_circ.1].push(bx.0);
            connection_count += 1;
        } else if a_in_circ.0 && b_in_circ.0 {
            // continue;
            let mut new_circuits = vec![];
            let mut merged_circuit = vec![];

            for i in 0..circuits.len() {
                if i == a_in_circ.1 || i == b_in_circ.1 {
                    for j in &circuits[i] {
                        merged_circuit.push(j.clone())
                    }
                    continue;
                }
                new_circuits.push(circuits[i].clone())
            }
            println!(
                "MER {:?} circuit {}, {}",
                merged_circuit, bx.0, bx.1,
            );
            new_circuits.push(merged_circuit);
            circuits = new_circuits;
        } else {
            println!(
                "NEW {:?} AND {:?} are in circuit",
                &boxes[bx.0], &boxes[bx.1]
            );
            // println!(
            //     "NEW {:?} AND {:?} are in circuit",
            //     bx.0, bx.1
            // );
            connection_count += 1;
            circuits.push(vec![bx.0, bx.1])
        }
    }

    println!("adding remaining boxes to singular circuits...");
    for bx in 0..boxes.len() {
        let mut in_circ = false;
        for circ in &circuits {
            if circ.contains(&bx) {
                in_circ = true;
                break;
            }
        }

        if !in_circ {
            circuits.push(vec![bx])
        }
    }

    for circ in &mut circuits {
        circ.sort();
        circ.dedup()
    }

    let mut total = 0;
    for circ in &circuits {
        total += circ.len()
    }
    if total != boxes.len() {
        panic!("something broke")
    }

    println!("{}: {:?}", circuits.len(), circuits);
    println!("{:?} connections", connection_count);

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|i| i.len()).collect();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    println!("{:?} connection sizes", circuit_sizes);

    for i in 0..3 {
        part1 *= circuit_sizes[i]
    }

    println!("part1: {}", part1);
}

fn get_distance(box_a: Vec<i64>, box_b: Vec<i64>) -> f64 {
    let dx = box_a[0] as f64 - box_b[0] as f64;
    let dy = box_a[1] as f64 - box_b[1] as f64;
    let dz = box_a[2] as f64 - box_b[2] as f64;

    (dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)).sqrt()
}

fn get_boxes(path: &str) -> Vec<Vec<i64>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut boxes: Vec<Vec<i64>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let parts: Vec<i64> = line
            .split(",")
            .filter_map(|i| i.trim().parse::<i64>().ok())
            .collect();

        boxes.push(parts)
    }

    boxes
}

// box_a, box_b, distance
fn get_box_distances(boxes: Vec<Vec<i64>>) -> Vec<(usize, usize, f64)> {
    let mut box_distances: Vec<(usize, usize, f64)> = vec![];
    let mut already_compared: Vec<(usize, usize)> = vec![];

    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }

            let dist = get_distance(boxes[i].clone(), boxes[j].clone());
            box_distances.push((i, j, dist));
            already_compared.push((i, j))
        }
    }
    box_distances.sort_by(|a, b| a.2.total_cmp(&b.2));
    box_distances
    
}
