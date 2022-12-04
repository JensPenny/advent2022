use std::fs;

use days::day1;
use days::day2;
use days::day3::day_3_a;

use crate::days::day3::day_3_b;
use crate::days::day4::day_4_a;
use crate::days::day4::day_4_b;

pub mod tools;
pub mod days;

fn main() {
    //day1();
    //day2();
    //day3();
    day4();
}

#[allow(dead_code)]
fn day4() {
    let input = fs::read_to_string("res/day4").expect("could not read day4");
    let day4 = day_4_a(&input);
    println!("Day 4 a: {day4}");

    let day4 = day_4_b(&input);
    println!("Day 4 b: {day4}");
}

#[allow(dead_code)]
fn day3() {
    let input = fs::read_to_string("res/day3").expect("could not read day3");
    let day3 = day_3_a(&input);
    println!("Day 3 a: {day3}");

    let day3 = day_3_b(&input);
    println!("Day 3 b: {day3}")
}

#[allow(dead_code)]
fn day2() {
    let input = fs::read_to_string("res/day2").expect("could not read day2");
    let day2 = day2::day_2_a(&input);
    println!("Day 2A: {day2}");

    let day2 = day2::day_2_b(&input);
    println!("Day 2B: {day2}");

}

#[allow(dead_code)]
fn day1() {
    let day1input = fs::read_to_string("res/day1").expect("Could not read file :(");
    let day1a = day1::day_1_a(&day1input, 1)[0];
    println!("Day 1A: {day1a}");
    let day1b:u32 = day1::day_1_a(&day1input, 3).iter().sum();
    println!("Day 1B: {day1b}");
}