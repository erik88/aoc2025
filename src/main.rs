mod day1;
mod day2;

use day1::day1;
use day2::day2;

use std::env;
use std::fs::read_to_string;

fn main() {
    let (day, test) = parse_args();

    let lines = read_lines(&format!("input/day{}{}.txt", day, test));

    match day.as_str() {
        "01" => day1(lines),
        "02" => day2(lines),
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
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .filter(|s| !s.is_empty())
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
