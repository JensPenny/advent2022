use std::str::FromStr;

#[derive(Debug)]
struct Backpack {
    front: String,
    back: String,
}

impl FromStr for Backpack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let middle = s.trim().len() / 2;
        let split = s.trim().split_at(middle);
        let pack = Self {
            front: split.0.to_string(),
            back: split.1.to_string(),
        };
        Ok(pack)
    }
}
impl Backpack {
    fn find_dupes(&self) -> String {
        let mut backpack_items = self.back.to_owned();
        let mut common_items = String::from("");
        for char in self.front.chars() {
            if backpack_items.contains(char) {
                common_items.push(char);
                let found_index = backpack_items.find(char);
                match found_index {
                    Some(_) => backpack_items = backpack_items.replace(&char.to_string(), ""), //backpack_items.remove(index), //Too diligent, copies may be removed -_-
                    None => (),
                };
            }
        }

        common_items
    }
}

fn calculate_score(input: &str) -> u32 {
    input
        .chars()
        .map(|c| -> u32 {
            //println!("mapping {}", c);
            match c {
                'A'..='Z' => (c as u32) - 64 + 26,
                'a'..='z' => (c as u32) - 96,
                _ => 0,
            }
        })
        .sum()
}

pub fn day_3_a(input: &String) -> u32 {
    input
        .lines()
        .map(|line| -> Backpack { Backpack::from_str(line).expect("no pack :(") })
        .map(|backpack| -> String { backpack.find_dupes() })
        .map(|dupe| -> u32 { calculate_score(&dupe) })
        .sum()
}

pub fn day_3_b(input: &String) -> u32 {
    let lines = input
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let chunky = lines.chunks(3);

    let mut total_score: u32 = 0;
    for chunk in chunky {
        let common_items = chunk
            .iter()
            .map(|s| s.to_string())
            .reduce(|acc, curr| acc.chars().filter(|char| curr.contains(*char)).collect())
            .expect("no common items? ");
        //println!("Common items: {common_items}");

        //Keep uniques - aka the first item :>
        let unique = common_items
            .chars()
            .next()
            .expect("no elements?")
            .to_string();

        let score = calculate_score(&unique);
        total_score += score;
    }
    total_score
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{day_3_a, day_3_b, Backpack};

    fn input() -> String {
        "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string()
    }

    #[test]
    fn test_3_b() {
        let result = day_3_b(&input());
        assert_eq!(result, 70)
    }

    #[test]
    fn test_3_a() {
        let sum = day_3_a(&input());
        assert_eq!(sum, 157);
    }

    #[test]
    fn test_compartiments() {
        let input = input()
            .lines()
            .next()
            .expect("Couldn't get first line :(")
            .to_string();
        let backpack = Backpack::from_str(&input).expect("burp");
        assert_eq!(backpack.front, "vJrwpWtwJgWr");
        assert_eq!(backpack.back, "hcsFMMfFFhFp");
    }

    #[test]
    fn test_find_dupes() {
        let input = input()
            .lines()
            .next()
            .expect("Couldn't get first line :(")
            .to_string();
        let backpack = Backpack::from_str(&input).expect("donk");
        assert_eq!(backpack.find_dupes(), "p");
    }
}
