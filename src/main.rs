use std::fs;
use std::time::Instant;

use days::day1;
use days::day2;
use days::day3::day_3_a;

use crate::days::day10::day_10_a;
use crate::days::day10::day_10_b;
use crate::days::day3::day_3_b;
use crate::days::day4::day_4_a;
use crate::days::day4::day_4_b;
use crate::days::day5::day_5_a;
use crate::days::day5::day_5_b;
use crate::days::day6::day_6;
use crate::days::day8::day_8_a;
use crate::days::day8::day_8_b;
use crate::days::day9::day_9_a;
use crate::days::day9::day_9_b;

pub mod tools;
pub mod days;

fn main() {
    let start = Instant::now();
    //day1();
    //day2();
    //day3();
    //day4();
    //day5();
    //day6();
    //day7();
    //day8();
    //day9();
    day10();

    let duration = start.elapsed();
    println!("solved in: {:?}", duration);
}

#[allow(dead_code)]
fn day10() {
    let input = fs::read_to_string("res/day10").expect("could not read day10");
    let day = day_10_a(&input);
    println!("Day 10 a: {day}");

    let day = day_10_b(&input);
    println!("Day 10 b: {day}");
}

#[allow(dead_code)]
fn day9() {
    let input = fs::read_to_string("res/day9").expect("could not read day9");
    let day = day_9_a(&input);
    println!("Day 9 a: {day}");

    let day = day_9_b(&input);
    println!("Day 9 b: {day}");
}

#[allow(dead_code)]
fn day8() {
    let input = fs::read_to_string("res/day8").expect("could not read day8");
    let day = day_8_a(&input);
    println!("Day 8 a: {day}");

    let day = day_8_b(&input);
    println!("Day 8 b: {day}");

}

#[allow(dead_code)]
fn day7() {
    //lol no. Leave me alone, trees in rust
}

#[allow(dead_code)]
fn day6() {
    let input = fs::read_to_string("res/day6").expect("could not read day6");
    let day = day_6(&input, 4);
    println!("Day 6 a: {day}");

    let day = day_6(&input, 14);
    println!("Day 6 b: {day}");
}

#[allow(dead_code)]
fn day5() {
    let input = fs::read_to_string("res/day5").expect("could not read day5");
    let day = day_5_a(&input);
    println!("Day 5 a: {day}");

    let day = day_5_b(&input);
    println!("Day 5 b: {day}");
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