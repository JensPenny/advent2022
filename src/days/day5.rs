use std::{result, str::Lines};

use itertools::{Chunk, Itertools};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Command {
    stack_to_move: usize,
    from: usize,
    to: usize,
}

fn day_5_a(input: &String) -> Vec<char> {
    let mut result: Vec<char> = vec![];

    let lines = &mut input.lines();
    let empty_index = lines
        .find_position(|line| line.is_empty())
        .expect("no empty line in the export");

    let input_string = &input
        .lines()
        .take(empty_index.0)
        .map(|s| s.to_string())
        .collect_vec();

    let stack_input = input_string.join("\n"); //Why do this you say? because I suck at rust. Any questions?
    let mut stacks = parse_stacks(&stack_input);

    let command_string = &input.lines().dropping(empty_index.0).map( |s| s.to_string()).collect_vec();
    let command_input = command_string.join("\n");
    let commands = parse_commands(&command_input);

    let res = do_day_a(&mut stacks, commands);

    result
}

fn do_day_a(stacks: &mut Vec<Vec<char>>, commands: Vec<Command>) -> String {
    for command in commands {
        let from_stack = stacks.get_mut(command.from - 1).expect("could not get from-stack");
        let to_stack = stacks.get_mut(command.to - 1).expect("could not get to-stack");
        for i in 1..(command.stack_to_move) {
            let to_move = from_stack.remove(0);
            to_stack.insert(0, to_move);
        }
    }

    "".to_string()
}

fn parse_stacks(input: &String) -> Vec<Vec<char>> {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect_vec();

    let mut result: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let chunks = line.chars().into_iter().chunks(4);

        for (index, mut chunk) in chunks.into_iter().enumerate() {
            if result.len() < index + 1 {
                result.push(Vec::new())
            }

            let current = result
                .get_mut(index)
                .expect("should always return something");

            let element = chunk.find_position(|ch| ch.is_alphabetic());
            match element {
                Some((_, char)) => current.push(char),
                None => (), //noop
            }
        }
    }

    result
}

fn parse_commands(input: &String) -> Vec<Command> {
    let mut result: Vec<Command> = vec![];

    let digits = Regex::new(r"\d+").unwrap();
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect_vec();
    for line in lines {
        let numbers = digits
            .find_iter(&line)
            .filter_map(|digits| digits.as_str().parse::<usize>().ok())
            .collect_vec();
        //println!("{line}");
        if numbers.len() != 3 {
            println!("{:#?}", numbers)
        } else {
            result.push(Command {
                stack_to_move: numbers[0],
                from: numbers[1],
                to: numbers[2],
            })
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::parse_stacks;
    use crate::days::day5::{parse_commands, Command};
    use itertools::Itertools;

    fn input() -> String {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string()
    }

    #[test]
    fn test_input_parser() {
        let input = input();
        let lines = &mut input.lines();
        let empty_index = lines
            .find_position(|line| line.is_empty())
            .expect("no empty line in the export");

        let input_string = &input
            .lines()
            .take(empty_index.0)
            .map(|s| s.to_string())
            .collect_vec();
        //let command_string = &input.lines().dropping(empty_index.0).map( |s| s.to_string()).collect_vec();

        let derp = input_string.join("\n"); //Why do this you say? because I suck at rust. Any questions?
        let stacks = parse_stacks(&derp);

        assert_eq!(*stacks.get(0).expect("no 1st stack"), vec!['N', 'Z']);
        assert_eq!(*stacks.get(1).expect("no 2nd stack"), vec!['D', 'C', 'M']);
        assert_eq!(*stacks.get(2).expect("no 3rd stack"), vec!['P']);

    }

    #[test]
    fn test_input_command() {
        let input = input();
        let lines = &mut input.lines();
        let empty_index = lines
            .find_position(|line| line.is_empty())
            .expect("no empty line in the export");

        //let input_string = &input.lines().take(empty_index.0).map(|s| s.to_string()).collect_vec();
        let command_string = &input
            .lines()
            .dropping(empty_index.0)
            .map(|s| s.to_string())
            .collect_vec();

        let derp = command_string.join("\n"); // Don't ask, the answer hasn't changed
        let commands = parse_commands(&derp);

        assert_eq!(
            *commands.get(0).expect("no 1st stack"),
            Command {
                stack_to_move: 1,
                from: 2,
                to: 1
            }
        );
    }
}
