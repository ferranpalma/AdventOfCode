use rust2023::{day01, day02, day03, day04, day05, day05random};
use std::fs;

fn main() {
    let data = fs::read_to_string("mock_data/day05.txt").expect("Can't read file");
    // day01::run(&data);
    // day02::run(&data);
    // day03::run(&data);
    // day04::run(&data);
    day05::run(&data); // Works with mock data, with real data it does not seem to finish lmao
}
