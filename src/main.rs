use std::fs;

use days::day1;

pub mod tools;
pub mod days;

fn main() {
    let day1input = fs::read_to_string("res/day1").expect("Could not read file :(");
    let day1a = day1::day_1_a(&day1input, 1)[0];
    println!("Day 1A: {day1a}");
    let day1b:u32 = day1::day_1_a(&day1input, 3).iter().sum();
    println!("Day 1B: {day1b}");
}
