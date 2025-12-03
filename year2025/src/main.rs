use std::{env, time::Instant};

mod days;
use days::{day01, day02, day03};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day> [test]");
        return;
    }

    let day = &args[1].parse::<u8>().unwrap_or(0);
    let is_test = args.get(2).is_some_and(|s| s == "test");
    let file_path = if is_test {
        format!("year2025/input_test/{day:02}.txt")
    } else {
        format!("year2025/input/{day:02}.txt")
    };

    let now = Instant::now();

    match day {
        1 => day01::solve(&file_path),
        2 => day02::solve(&file_path),
        3 => day03::solve(&file_path),
        4..=12 => println!("Day {day} not implemented yet."),
        13..=25 => println!("Only 12 days in AoC 2025!"),
        _ => eprintln!("Invalid day: {day}"),
    }

    let elapsed = now.elapsed();
    println!("Day {}{} ran in {:?}", day, if is_test {" test"} else {""}, elapsed);
}
