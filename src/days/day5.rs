use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Command {
    stack_to_move: usize,
    from: usize,
    to: usize,
}

pub fn day_5_a(input: &String) -> String {
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

    let command_string = &input
        .lines()
        .dropping(empty_index.0)
        .map(|s| s.to_string())
        .collect_vec();
    let command_input = command_string.join("\n");
    let commands = parse_commands(&command_input);

    let res = do_day_a(&mut stacks, commands);

    res
}

//A lot of copy pasting, but mostly because I'm tired and don't want to refactor.
pub fn day_5_b(input: &String) -> String {
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

    let command_string = &input
        .lines()
        .dropping(empty_index.0)
        .map(|s| s.to_string())
        .collect_vec();
    let command_input = command_string.join("\n");
    let commands = parse_commands(&command_input);

    let res = do_day_b(&mut stacks, commands);

    res
}

//More copy pasting, it's not illegal, so sue me!!.. (pls don't)
fn do_day_b(stacks: &mut Vec<VecDeque<char>>, commands: Vec<Command>) -> String {
    for command in commands {
        let mut holder: VecDeque<char> = VecDeque::new();

        for stack in stacks.iter_mut().enumerate() {
            if stack.0 == command.from - 1 {
                for _n in 0..(command.stack_to_move) {
                    holder.push_front(stack.1.pop_front().expect("cant pop"));
                }
            }
        }

        for stack in stacks.iter_mut().enumerate() {
            if stack.0 == command.to - 1 {
                for held in holder.iter() {
                    stack.1.push_front(*held);
                }
            }
        }
    }

    let mut result = String::from("");
    stacks
        .iter_mut()
        .for_each(|dequeu| result.push(dequeu.pop_front().expect("cant pop 2")));
    result.to_string()
}

fn do_day_a(stacks: &mut Vec<VecDeque<char>>, commands: Vec<Command>) -> String {
    for command in commands {
        let mut holder: VecDeque<char> = VecDeque::new();

        for stack in stacks.iter_mut().enumerate() {
            if stack.0 == command.from - 1 {
                for _n in 0..(command.stack_to_move) {
                    holder.push_back(stack.1.pop_front().expect("cant pop"));
                }
            }
        }

        for stack in stacks.iter_mut().enumerate() {
            if stack.0 == command.to - 1 {
                for held in holder.iter() {
                    stack.1.push_front(*held);
                }
            }
        }
    }

    let mut result = String::from("");
    stacks
        .iter_mut()
        .for_each(|dequeu| result.push(dequeu.pop_front().expect("cant pop 2")));
    result.to_string()
}

fn parse_stacks(input: &String) -> Vec<VecDeque<char>> {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect_vec();

    let mut result: Vec<VecDeque<char>> = Vec::new();

    for line in lines {
        let chunks = line.chars().into_iter().chunks(4);

        for (index, mut chunk) in chunks.into_iter().enumerate() {
            if result.len() < index + 1 {
                result.push(VecDeque::new())
            }

            let current = result
                .get_mut(index)
                .expect("should always return something");

            let element = chunk.find_position(|ch| ch.is_alphabetic());
            match element {
                Some((_, char)) => current.push_back(char),
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
    use super::{day_5_a, parse_stacks};
    use crate::days::day5::{parse_commands, Command, day_5_b};
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
    fn test_day_b() {
        let input = input();
        let result = day_5_b(&input);

        assert_eq!(result, "MCD");
    }

    #[test]
    fn test_day_a() {
        let input = input();
        let result = day_5_a(&input);

        assert_eq!(result, "CMZ");
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
