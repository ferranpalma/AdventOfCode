use rust2023::{day01, day02, day03, day04};
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day04.txt").expect("Can't read file");
    // day01::run(&data);
    // day02::run(&data);
    // day03::run(&data);
    day04::run(&data)
}
