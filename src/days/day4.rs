use std::str::FromStr;

// The following struct is for the activity.
#[derive(Debug, PartialEq)]
struct ElfPair(Elf, Elf);

#[derive(Debug, PartialEq)]
struct Elf {
    start: u8,
    end: u8,
}

impl Elf {
    fn fully_contains(&self, other: &Elf) -> bool {
        (self.start <= other.start && self.end >= other.end)
        || (self.start >= other.start && self.end <= other.end)
    }

    fn doesnt_overlap(&self, other: &Elf) -> bool {
        (self.end < other.start) || (other.end < self.start)
    }
}

impl FromStr for Elf {
    type Err = ();

    //Would be neater to throw back exceptions when failing, but /shrug
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let elf = Self {
            start: split
                .next()
                .expect("first item not found")
                .parse()
                .expect("could not parse first item"),
            end: split
                .next()
                .expect("first item not found")
                .parse()
                .expect("could not parse first item"),
        };

        Ok(elf)
    }
}

fn make_pairs(input: &String) -> Vec<ElfPair> {
    let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();
    let pairs: Vec<ElfPair> = lines
        .iter()
        .map(|line| -> ElfPair {
            let split: Vec<&str> = line.split(",").collect();

            let first = *split.get(0).expect(":(");
            let elf1 = Elf::from_str(first).expect("herp");

            let second = split.get(1).expect(":(");
            let elf2 = Elf::from_str(second).expect("derp");

            ElfPair(elf1, elf2)
        })
        .collect();

    pairs
}

pub fn day_4_a(input: &String) -> usize {
    let pairs = make_pairs(input);

    let filtered: Vec<ElfPair> = pairs.into_iter().filter(|pair| -> bool {
        pair.0.fully_contains(&pair.1)
    }).collect();

    filtered.len()
}

pub fn day_4_b(input: &String) -> usize {
    let pairs = make_pairs(input);

    let filtered: Vec<ElfPair> = pairs.into_iter().filter(|pair| -> bool {
        !(pair.0.doesnt_overlap(&pair.1))
    }).collect();

    filtered.len()
}

#[cfg(test)]
mod tests {
    use super::{day_4_a, make_pairs, Elf, ElfPair, day_4_b};

    fn input() -> String {
        "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
        .to_string()
    }

    #[test]
    fn test_day_a() {
        assert_eq!(2, day_4_a(&input()))
    }

    #[test]
    fn test_day_b() {
        assert_eq!(4, day_4_b(&input()))
    }

    #[test]
    fn test_pairing_func() {
        let test_input = input();
        let pairs = make_pairs(&test_input);

        assert_eq!(
            pairs[0],
            ElfPair(Elf { start: 2, end: 4 }, Elf { start: 6, end: 8 })
        )
    }
}
