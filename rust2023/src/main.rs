use rust2023::{day01, day02};
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day02.txt").expect("Can't read file");
    // day01::run(&data);
    day02::run(&data);
}
