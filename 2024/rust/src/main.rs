use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: aoc2024 <day>\nexample: aoc2024 1")
    }
    let _ = args[1]
        .parse::<i8>()
        .map_or_else(|_| println!("please use a valid day"), execute);
}

fn execute(day: i8) {
    match day {
        1 => day1::execute(),
        2 => println!("not implemented"),
        3 => println!("not implemented"),
        4 => println!("not implemented"),
        5 => println!("not implemented"),
        6 => println!("not implemented"),
        7 => println!("not implemented"),
        8 => println!("not implemented"),
        9 => println!("not implemented"),
        10 => println!("not implemented"),
        11 => println!("not implemented"),
        12 => println!("not implemented"),
        _ => println!("not available"),
    }
}
