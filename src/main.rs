mod day1;
mod day2;


use std::fs::read_to_string;
use day1::day1;
use day2::day2;

fn main() {
    let lines = read_lines("input/day02.txt");

    //day1(lines);
    day2(lines);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .filter(|s| !s.is_empty())
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
