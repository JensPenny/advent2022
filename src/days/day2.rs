use std::str::FromStr;

#[derive(Debug)]
enum Combo {
    RR, 
    RP,
    RS, 

    PR, 
    PP,
    PS,
    SR,
    SP,
    SS,
}

impl FromStr for Combo {
    type Err = ();

    fn from_str(input: &str) -> Result<Combo, Self::Err> {
        match input {
            "A X"  => Ok(Combo::RR),
            "A Y"  => Ok(Combo::RP),
            "A Z"  => Ok(Combo::RS),

            "B X"  => Ok(Combo::PR),
            "B Y"  => Ok(Combo::PP),
            "B Z"  => Ok(Combo::PS),

            "C X"  => Ok(Combo::SR),
            "C Y"  => Ok(Combo::SP),
            "C Z"  => Ok(Combo::SS),
            _      => Err(()),
        }
    }
}

fn score_day_a(combo: Combo) -> u8 {
    match combo {
        Combo::RR => 1 + 3,
        Combo::RP => 2 + 6,
        Combo::RS => 3 + 0,
        Combo::PR => 1 + 0,
        Combo::PP => 2 + 3,
        Combo::PS => 3 + 6,
        Combo::SR => 1 + 6,
        Combo::SP => 2 + 0,
        Combo::SS => 3 + 3,
    }
}

fn score_day_b(combo: Combo) -> u8 {
    match combo {
        Combo::RR => 0 + 3,
        Combo::RP => 3 + 1,
        Combo::RS => 6 + 2,
        Combo::PR => 0 + 1,
        Combo::PP => 3 + 2,
        Combo::PS => 6 + 3,
        Combo::SR => 0 + 2,
        Combo::SP => 3 + 3,
        Combo::SS => 6 + 1,
    }
}

pub fn day_2_a(input: &String) -> u32 {
    input.lines()
        .map(|line| Combo::from_str(line.trim()).expect("error converting :("))
        .map(|combo| score_day_a(combo))
        .map(|score| score as u32) 
        .sum()
}

pub fn day_2_b(input: &String) -> u32 {
    input.lines()
        .map(|line| Combo::from_str(line.trim()).expect("error converting :("))
        .map(|combo| score_day_b(combo))
        .map(|score| score as u32) 
        .sum()
}
 

#[cfg(test)]
mod tests {
    use super::day_2_a;
    use super::day_2_b;

    #[test]
    fn test_2_a() {
        let input = "A Y
        B X
        C Z".to_string();
        let result = day_2_a(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_2_b() {
        let input = "A Y
        B X
        C Z".to_string();
        let result = day_2_b(&input);
        assert_eq!(result, 12);
    }

}