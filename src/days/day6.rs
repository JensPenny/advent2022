use std::collections::HashSet;

use itertools::Itertools;


pub fn day_6(input: &String, slice_size: usize) -> usize {

    for i in slice_size..input.len() + 1 {
        let slice_dice = &input[i-slice_size..i];
        if slice_dice.as_bytes().iter().all_unique() { //lol thanks itertools. Almost(*) feels like cheating
            return i
        }
        //println!("sliced {slice_dice}");
    };
    return 0;
}

pub fn day_6_no_iter(input: &String, slice_size: usize) -> usize {

    for i in slice_size..input.len() + 1 {
        let slice_dice = &input[i-slice_size..i];
        
        let set:HashSet<&u8> = HashSet::from_iter(slice_dice.as_bytes().iter());
        if set.len() == slice_size {
            return i
        }
        //println!("sliced {slice_dice}");
    };
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{day_6, day_6_no_iter};


    fn input1() -> String {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()
    }

    #[test]
    fn test_day_6_a() {
        let input = input1();
        let result = day_6(&input, 4);
        assert_eq!(result, 7)
    }

    #[test]
    fn test_day_6_b() {
        let input = input1();
        let result = day_6(&input, 14);
        assert_eq!(result, 19)
    }

    #[test]
    fn test_day_6_b_no_iter() {
        let input = input1();
        let result = day_6_no_iter(&input, 14);
        assert_eq!(result, 19)
    }

}