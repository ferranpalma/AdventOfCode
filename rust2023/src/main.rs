use rust2023::{day01, day02, day03};
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day03.txt").expect("Can't read file");
    // day01::run(&data);
    // day02::run(&data);
    day03::run(&data)
}
