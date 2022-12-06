use itertools::Itertools;


pub fn day_6_a(input: &String) -> usize {

    for i in 4..input.len() + 1 {
        let slice_dice = &input[i-4..i];
        if slice_dice.as_bytes().iter().all_unique() { //lol thanks itertools. Almost(*) feels like cheating
            return i
        }
        //println!("sliced {slice_dice}");
    };
    return 0;
}

#[cfg(test)]
mod tests {
    use super::day_6_a;


    fn input1() -> String {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()
    }

    #[test]
    fn test_day_6_a() {
        let input = input1();
        let result = day_6_a(&input);
        assert_eq!(result, 7)
    }

}