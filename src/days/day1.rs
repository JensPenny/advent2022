use std::num::ParseIntError;

pub fn day_1_a(input: &String) -> u32 {
    let result: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut max: u32 = 0;
    let mut current: u32 = 0;
    for line in result {
        let calories: Result<u32, ParseIntError> = line.trim().parse();
        match calories {
            Ok(num) => current += num,
            Err(_) => {
                if current > max {
                    max = current;
                }
                current = 0;
            },
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::day_1_a;

    #[test]
    fn test_day_a() {
        let result = day_1_a(&"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ".to_string());
        assert_eq!(result, 24000);

    }

}