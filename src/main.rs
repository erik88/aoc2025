mod board;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;

use std::env;
use std::fs::read_to_string;

fn main() {
    let (day, test) = parse_args();

    let lines = read_lines(&format!("input/day{}{}.txt", day, test));

    match day.as_str() {
        "01" => day1(lines),
        "02" => day2(lines),
        "03" => day3(lines),
        "04" => day4(lines),
        "05" => day5(lines),
        "06" => day6(lines),
        "07" => day7(lines),
        "08" => day8(lines, !test.is_empty()),
        "09" => day9(lines),
        "10" => day10(lines),
        "11" => day11(lines, !test.is_empty()),
        "12" => day12(lines),
        _ => println!("Invalid day {}, quitting.", day),
    }
}

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Requires arguments: [day] [optional: test]");
        std::process::exit(1);
    }

    let day = if args[1].len() == 1 {
        format!("0{}", args[1])
    } else {
        args[1].clone()
    };

    let test = match args.get(2).map(|s| s.as_str()) {
        Some("test") => ".test",
        _ => "",
    }
    .to_owned();

    (day, test)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut lines: Vec<String> = read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect(); // gather them together into a vector

    if lines.last().unwrap().is_empty() {
        lines.pop();
    }

    lines
}
