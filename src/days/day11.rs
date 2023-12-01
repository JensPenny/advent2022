use itertools::Itertools;

struct Monkey {
    inspected_amount: u8,
    items: Vec<u8>,
    interest_operation: fn(i32) -> i32,

    divisible_test: i32,
    true_target: usize,
    false_target: usize,
}

fn parse(input: &String) -> Vec<Monkey> {
    let monke_functions: Vec<fn(i32) -> i32> = vec![monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7];

    let chunky = input.lines().chunks(7);
    let monkeys = chunky.into_iter().collect_vec();

    let result: Vec<Monkey> = Vec::new();
    for monkey in monkeys.into_iter().enumerate() {
        let collected = monkey.1.into_iter().collect_vec();
        let starting_item_str = collected[1];
        let test_parameter = collected[3];
        let true_param = collected[4];
        let false_param = collected[5];
        let operation = monke_functions[monkey.0];

        println!("Starting items: {starting_item_str}");
        println!("Test param: {test_parameter}");
        println!("True param: {true_param }");
        println!("False param: {false_param }");
        println!("Operation: {:#?}", operation);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::parse;


    fn input() -> String {
        "Monkey 0:
Starting items: 64, 89, 65, 95
Operation: new = old * 7
Test: divisible by 3
  If true: throw to monkey 4
  If false: throw to monkey 1
      
Monkey 1:
  Starting items: 76, 66, 74, 87, 70, 56, 51, 66
  Operation: new = old + 5
  Test: divisible by 13
    If true: throw to monkey 7
    If false: throw to monkey 3
    ".to_string()
    }

    #[test]
    fn test_parse() {
        let input = input();
        let parsed = parse(&input);

        assert_eq!(parsed[0].false_target, 1);
        assert_eq!(parsed[1].true_target, 4)
    }
}

//Monke-functions. A but useless but hey
fn monkey0(old: i32) -> i32 {
    old * 7
}

fn monkey1(old: i32) -> i32 {
    old + 5
}

fn monkey2(old: i32) -> i32 {
    old * old
}

fn monkey3(old: i32) -> i32 {
    old + 6
}

fn monkey4(old: i32) -> i32 {
    old * 11
}

fn monkey5(old: i32) -> i32 {
    old + 8
}

fn monkey6(old: i32) -> i32 {
    old + 1
}

fn monkey7(old: i32) -> i32 {
    old + 4
}
