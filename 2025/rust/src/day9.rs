use macroquad::prelude::*;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::thread;
use std::time::Duration;

pub async fn execute() {
    let points = get_points("inputs/day9");

    loop {
        clear_background(WHITE);
        let mut part1 = 0; //part1(points.clone());
        let fpoints = divide_all_points_by(points.clone(), 80);
        for (idx, point) in points.iter().enumerate() {
            clear_background(WHITE);
            draw_circle(fpoints[idx].0, fpoints[idx].1, 2.0, RED);
            for (other_idx, other_point) in points.iter().skip(idx + 1).enumerate() {
                draw_line(
                    fpoints[idx].0,
                    fpoints[idx].1,
                    fpoints[other_idx].0,
                    fpoints[other_idx].1,
                    2.0,
                    RED,
                );
                thread::sleep(Duration::from_millis(500));
                next_frame().await;
                if point.0 == other_point.0 || point.1 == other_point.1 {
                    continue;
                }

                let x_min = point.0.min(other_point.0);
                let x_max = point.0.max(other_point.0);
                let y_min = point.1.min(other_point.1);
                let y_max = point.1.max(other_point.1);

                let width = x_max - x_min + 1;
                let height = y_max - y_min + 1;
                let area = width * height;
                if area > part1 {
                    part1 = area;
                }
            }
        }
        // let part2 = part2(points.clone());
    }
}

async fn part1(points: Vec<(i64, i64)>) -> i64 {
    let mut part1 = 0;

    let fpoints = divide_all_points_by(points.clone(), 80);
    draw_text("PART 1", 20.0, 20.0, 20.0, BLACK);
    for (idx, point) in points.iter().enumerate() {
        draw_circle(fpoints[idx].0, fpoints[idx].1, 2.0, RED);
        for (other_idx, other_point) in points.iter().skip(idx + 1).enumerate() {
            draw_line(
                fpoints[idx].0,
                fpoints[idx].1,
                fpoints[other_idx].0,
                fpoints[other_idx].1,
                2.0,
                RED,
            );
            thread::sleep(Duration::from_millis(500));
            next_frame().await;
            if point.0 == other_point.0 || point.1 == other_point.1 {
                continue;
            }

            let x_min = point.0.min(other_point.0);
            let x_max = point.0.max(other_point.0);
            let y_min = point.1.min(other_point.1);
            let y_max = point.1.max(other_point.1);

            let width = x_max - x_min + 1;
            let height = y_max - y_min + 1;
            let area = width * height;
            if area > part1 {
                part1 = area;
            }
        }
    }
    part1
}

fn get_points(path: &str) -> Vec<(i64, i64)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut points: Vec<(i64, i64)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        points.push((x, y));
    }

    points
}

#[derive(Debug)]
struct Scanline {
    y_sample: f64,
    spans: Vec<(i64, i64)>,
}

fn part2(points: Vec<(i64, i64)>) -> i64 {
    let mut part2 = 0;

    let mut unique_y: Vec<i64> = points.to_vec().iter().map(|(_, y)| *y).collect();
    unique_y.sort();
    unique_y.dedup();

    let mut scanlines = vec![];
    for pair in unique_y.windows(2) {
        let y_sample = (pair[0] as f64 + pair[1] as f64) / 2.0;
        let spans = spans_at_y(y_sample, &points);
        scanlines.push(Scanline { y_sample, spans });
    }
    println!("{:?}", scanlines);

    for (idx, point) in points.iter().enumerate() {
        for other_point in points.iter().skip(idx + 1) {
            if point.0 == other_point.0 || point.1 == other_point.1 {
                continue;
            }

            let x_min = point.0.min(other_point.0);
            let x_max = point.0.max(other_point.0);
            let y_min = point.1.min(other_point.1);
            let y_max = point.1.max(other_point.1);

            if rectangle_is_inside_polygon(x_min, x_max, y_min, y_max, &scanlines) {
                let width = x_max - x_min + 1;
                let height = y_max - y_min + 1;
                let area = width * height;

                if area > part2 {
                    part2 = area;
                }
            }
        }
    }
    part2
}

fn rectangle_is_inside_polygon(
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    scanlines: &[Scanline],
) -> bool {
    for scanline in scanlines {
        if scanline.y_sample < y_min as f64 || scanline.y_sample > y_max as f64 {
            continue;
        }

        let mut covered = false;
        for &(start, end) in &scanline.spans {
            if x_min >= start && x_max <= end {
                covered = true;
                break;
            }
        }

        if !covered {
            return false;
        }
    }

    true
}

fn spans_at_y(y: f64, polygon: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut intersections: Vec<i64> = Vec::new();
    let len = polygon.len();

    for i in 0..len {
        let a = polygon[i];
        let b = polygon[(i + 1) % len];

        if a.1 == b.1 {
            continue;
        }

        let y1 = a.1 as f64;
        let y2 = b.1 as f64;

        if (y > y1.min(y2)) && (y <= y1.max(y2)) {
            intersections.push(a.0);
        }
    }

    intersections.sort();
    let mut spans = Vec::new();
    let mut iter = intersections.chunks_exact(2);
    while let Some(pair) = iter.next() {
        spans.push((pair[0], pair[1]));
    }
    spans
}

fn divide_all_points_by(points: Vec<(i64, i64)>, divisor: i64) -> Vec<(f32, f32)> {
    points
        .into_iter()
        .map(|(x, y)| (x as f32 / divisor as f32, y as f32 / divisor as f32))
        .collect()
}
