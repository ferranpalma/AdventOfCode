use rust2023::day01;
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day01.txt").expect("Can't read file");
    day01::run(&data);
}
