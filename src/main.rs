mod day1;

use day1::day1;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input/day01.txt");

    day1(lines);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .filter(|s| !s.is_empty())
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
