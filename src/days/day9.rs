use itertools::Itertools;
use std::{collections::HashSet, fmt::Debug, str::FromStr};

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    amount: u8,
}

#[derive(Debug, PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::LEFT),
            "R" => Ok(Direction::RIGHT),
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} - Y: {}", self.x, self.y)
        //f.debug_struct("Pos").field("x", &self.x).field("y", &self.y).finish()
    }
}

impl Pos {
    //add_to_pos with dir?
    fn do_single_move(&mut self, direction: &Direction) {
        match direction {
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
        }
    }

    fn move_toward(&mut self, head: &Pos) {
        //Check if we need to move
        if !self.need_to_move(head) {
            return;
        }

        //Do a move
        self.x += (head.x - self.x).signum();
        self.y += (head.y - self.y).signum();
    }
    fn need_to_move(&self, otherpos: &Pos) -> bool {
        (self.x - otherpos.x).abs() >= 2 || (self.y - otherpos.y).abs() >= 2
    }
}

pub fn day_9_a(input: &String) -> usize {
    let moves = parse(&input);
    let visits = do_moves(&moves, 2);

    visits.len()
}

pub fn day_9_b(input: &String) -> usize {
    let moves = parse(&input);
    let visits = do_moves(&moves, 10);

    visits.len()
}

fn do_moves(moves: &Vec<Move>, knots_to_make: u8) -> HashSet<Pos> {
    let mut knots: Vec<Pos> = Vec::new();
    for _ in 0..knots_to_make {
        knots.push(Pos { x: 0, y: 0 });
    }

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(knots[knots.len() - 1]);

    for mv in moves {
        for _ in 0..mv.amount {
            let mut head = knots[0];
            head.do_single_move(&mv.direction);
            knots[0] = head;

            //Move the rest of the knots
            for knot_index in 1..knots.len() {
                let former = knots[knot_index - 1];
                let mut tail = knots[knot_index];
                tail.move_toward(&former);
                knots[knot_index] = tail;

                if knot_index == (knots.len() - 1) {
                    visited.insert(tail); //implicit copy is a bit unintuitive imo :(
                    println!("H: {:#?} | T: {:#?}", former, tail);
                }
            }
        }
    }

    visited
}

fn parse(input: &String) -> Vec<Move> {
    input
        .lines()
        .map(|line| -> Move {
            let mut split = line.split(" ");
            Move {
                direction: Direction::from_str(split.next().expect("no more strings"))
                    .expect("cant make dir :("),
                amount: split
                    .next()
                    .expect("cant maek command")
                    .parse()
                    .expect("should parse"),
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::days::day9::Direction;

    use super::{do_moves, parse, Move};

    fn input() -> String {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .to_string()
    }

    #[test]
    fn test_parse() {
        let input = input();
        let moves = parse(&input);

        assert_eq!(
            moves[0],
            Move {
                amount: 4,
                direction: Direction::RIGHT,
            }
        );
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_day_a() {
        let input = input();
        let moves = parse(&input);

        let visits = do_moves(&moves, 2);
        assert_eq!(13, visits.len());
    }

    #[test]
    fn test_day_b_input_1() {
        let input = input();
        let moves = parse(&input);

        let visits = do_moves(&moves, 10);
        assert_eq!(1, visits.len());
    }

    fn larger_input() -> String {
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .to_string()
    }

    #[test]
    fn test_day_b_input_2() {
        let input = larger_input();
        let moves = parse(&input);

        let visits = do_moves(&moves, 10);
        assert_eq!(36, visits.len());
    }
}
