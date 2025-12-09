use std::env;

use macroquad::window::Conf;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn window_conf() -> Conf {
    Conf {
        window_title: "AOC 2025".to_owned(),
        window_width: 1280,
        window_height: 1280,
        high_dpi: true, 
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: aoc2025 <day>\nexample: aoc2025 1")
    }
    let day = args[1].parse::<i8>().unwrap_or_else(|_| {
        println!("please use a valid day");
        std::process::exit(1);
    });

    execute(day).await;
}

async fn execute(day: i8) {
    match day {
        1 => day1::execute(),
        2 => day2::execute(),
        3 => day3::execute(),
        4 => day4::execute(),
        5 => day5::execute(),
        6 => day6::execute(),
        7 => day7::execute(),
        8 => day8::execute(),
        9 => day9::execute().await,
        10 => println!("not implemented"),
        11 => println!("not implemented"),
        12 => println!("not implemented"),
        _ => println!("not available"),
    }
}
