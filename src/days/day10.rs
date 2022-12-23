use itertools::Itertools;

pub fn day_10_a(input: &String) -> i32 {
    let mut cycle: i32 = 1; //signed for easy multiplication in register
    let mut register: i32 = 1;

    let mut result = 0;
    let mut cycles_to_check = vec![20, 60, 100, 140, 180, 220, 1000000000];
    cycles_to_check.reverse();
    let mut next_cycle = cycles_to_check.pop().expect("found_next");

    for line in input.lines() {
        match line {
            "noop" => {
                cycle += 1;
            }
            line if line.starts_with("addx") => {
                let to_add = &line[5..line.len()];
                cycle += 2;

                if cycle > next_cycle {
                    let current = next_cycle * register;
                    result += current;
                    next_cycle = cycles_to_check.pop().expect("found next");
                }

                //Add register AFTER cycle, since we need to use the last register value
                register += to_add.parse::<i32>().expect("cant parse?");
            }
            _ => (),
        }
    }

    result
}

pub fn day_10_b(input: &String) -> i32 {
    let mut register: i32 = 1; //Pixel pointer

    let mut commands = input.lines().rev().collect_vec(); //Reverse to easily pop
    let mut last_command = commands.pop().expect("peut");

    let mut global_cycle = 1;
    let mut next_command_cycle = 0;

    //Initialize end for next command
    if last_command == "noop" {
        next_command_cycle = 1;
    } else {
        next_command_cycle = 2;
    }

    for _row in 0..6 {
        let mut row_buffer = "########################################".chars().into_iter().collect_vec();

        for cycle in 1..=40 {
            //println!("Cycle {cycle}");
            //println!("Register location {register}");
            //Draw pixel
            if ((&register - 1)..=(&register + 1)).contains(&(cycle - 1)) {
                row_buffer[cycle as usize- 1] = '#';
                //println!("drawing # on {}", cycle as usize - 1);
            } else {
                row_buffer[cycle as usize - 1] = '.';
                //println!("drawing . on {}", cycle as usize - 1);
            }
            //println!("Row: {}", row_buffer.clone().into_iter().collect::<String>());

            //Get command
            if next_command_cycle == global_cycle {
                match last_command {
                    "noop" => {
                        next_command_cycle += 1;
                        last_command = commands.pop().expect("blerp");
                    }, 
                    command if command.starts_with("addx") => {
                        let to_add = &command[5..command.len()];
                        next_command_cycle += 2; 
                        register += to_add.parse::<i32>().expect("cant parse?");
                        //println!("Executed command {}", last_command);
                        last_command = commands.pop().expect("blerp");
                    },
                    _ => (),
                }
            }

            //Update global cycle
            global_cycle += 1;
        }

        let line: String = row_buffer.into_iter().collect();
        println!("{line}");
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{day_10_a, day_10_b};

    #[test]
    fn test_day_a() {
        let input = input();
        let result = day_10_a(&input);

        assert_eq!(13140, result);
    }

    #[test]
    fn test_day_b() {
        let input = input();
        let result = day_10_b(&input);

        assert_eq!(0, result);
    }

    fn input() -> String {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            .to_string()
    }
}
