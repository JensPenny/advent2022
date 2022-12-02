use std::fs;

use days::day1;
use days::day2;

pub mod tools;
pub mod days;

fn main() {
    //day1();
    day2();
}

fn day2() {
    let input = fs::read_to_string("res/day2").expect("could not read day2");
    let day2 = day2::day_2_a(&input);
    println!("Day 2A: {day2}");

    let day2 = day2::day_2_b(&input);
    println!("Day 2B: {day2}");

}

fn day1() {
    let day1input = fs::read_to_string("res/day1").expect("Could not read file :(");
    let day1a = day1::day_1_a(&day1input, 1)[0];
    println!("Day 1A: {day1a}");
    let day1b:u32 = day1::day_1_a(&day1input, 3).iter().sum();
    println!("Day 1B: {day1b}");
}