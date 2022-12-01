use std::num::ParseIntError;

pub fn day_1_a(input: &String, length: usize) -> Vec<u32> {
    let result: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut maxes: Vec<u32> = Vec::with_capacity(length);
    let mut current: u32 = 0;
    for line in result {
        let calories: Result<u32, ParseIntError> = line.trim().parse();
        match calories {
            Ok(num) => current += num,
            Err(_) => {
                if maxes.len() < length {
                    maxes.push(current);
                } else {
                    //todo: fix met indices nerd
                    let min = maxes.iter_mut().min().expect("derp");
                    if current > *min {
                        *min = current;
                    }
                }
                current = 0;
            },
        }
    }
    maxes.iter().map(|f| *f).collect()
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
        ".to_string(), 1);
        assert_eq!(result[0], 24000);
    }

    #[test]
    fn test_day_b() {
        let mut result = day_1_a(&"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ".to_string(), 3);
        result.sort();
        assert_eq!(result[0], 10000);
        assert_eq!(result[1], 11000);
        assert_eq!(result[2], 24000);
    }


}