pub fn day_8_a(input: &String) -> usize {
    let mut grid = create_grid(input);
    count_full(&mut grid)
}

pub fn day_8_b(input: &String) -> usize {
    let grid = create_grid(input);
    println!("{:?}", &grid);
    find_scores(&grid)
}

fn find_scores(grid: &Vec<Vec<u8>>) -> usize {
    let size = grid.len();
    let mut max_score = 0;
    for x in 1..size - 1 {
        //We only consider internal trees
        for y in 1..size - 1 {
            let score = calculate_from_position(grid, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn calculate_from_position(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let size = grid.len(); //dont care for duplication
    let current_tree_score = grid[x][y];

    let mut left_score = 1;
    {
        //0 to x in reverse (from x to 0)
        for left in 1..x {
            let next_left = grid[x - left][y];
            if next_left < current_tree_score {
                left_score += 1;
            } else {
                break;
            }
        }
    }

    let mut right_score = 1;
    {
        //x to size(from x to max len)
        for right in 1..(size - x - 1) {
            let next_right = grid[x + right][y];
            if next_right < current_tree_score {
                right_score += 1;
            } else {
                break;
            }
        }
    }

    let mut top_score = 1;
    {
        //0 to y in reverse (from y to 0)
        for top in 1..y {
            let next_top = grid[x][y - top];
            if next_top < current_tree_score {
                top_score += 1;
            } else {
                break;
            }
        }
    }

    let mut bot_score = 1;
    {
        //x to size(from x to max len)
        for bot in 1..(size - y - 1) {
            let next_bot = grid[x][y + bot];
            if next_bot < current_tree_score {
                bot_score += 1;
            } else {
                break;
            }
        }
    }
    //y to size
    println!("Coord ({}-{}): L {} - R {} - T {} B {}",x, y, left_score, right_score, top_score, bot_score);
    left_score * right_score * top_score * bot_score
}

fn count_full(grid: &mut Vec<Vec<u8>>) -> usize {
    let size = grid.len();

    let mut counted_trees: Vec<(u8, u8)> = Vec::new();
    count_vertical(size, grid, &mut counted_trees);
    count_horizontal(size, grid, &mut counted_trees);

    counted_trees.len()
}

fn count_vertical(size: usize, grid: &mut Vec<Vec<u8>>, counted_trees: &mut Vec<(u8, u8)>) {
    //Watching from top to bottom
    for top in 0..size {
        let mut innerloop = 0;
        let mut last = grid[top][innerloop]; //x increments, y 0

        let current_tree = (top as u8, innerloop as u8);
        if !counted_trees.contains(&current_tree) {
            counted_trees.push(current_tree)
        }

        innerloop += 1;
        while innerloop < size {
            if last < grid[top][innerloop] {
                last = grid[top][innerloop];

                let current_tree = (top as u8, innerloop as u8);
                if !counted_trees.contains(&current_tree) {
                    counted_trees.push(current_tree)
                }
            }

            innerloop += 1;
        }
    }

    //Watching from bottom to top
    for bottom in 0..size {
        let mut innerloop = 0;
        let mut last = grid[bottom][size - 1 - innerloop]; //x increments, y 0

        let current_tree = (bottom as u8, (size - 1 - innerloop) as u8);
        if !counted_trees.contains(&current_tree) {
            counted_trees.push(current_tree)
        }

        innerloop += 1;
        while innerloop < size {
            if last < grid[bottom][size - 1 - innerloop] {
                last = grid[bottom][size - 1 - innerloop];

                let current_tree = (bottom as u8, (size - 1 - innerloop) as u8);
                if !counted_trees.contains(&current_tree) {
                    counted_trees.push(current_tree)
                }
            }

            innerloop += 1;
        }
    }
}

fn count_horizontal(size: usize, grid: &mut Vec<Vec<u8>>, counted_trees: &mut Vec<(u8, u8)>) {
    //Watching from left
    for left in 0..size {
        let mut innerloop = 0;
        let mut last = grid[innerloop][left]; //x increments, y 0

        let current_tree = (innerloop as u8, left as u8);
        if !counted_trees.contains(&current_tree) {
            counted_trees.push(current_tree)
        }

        innerloop += 1;
        while innerloop < size {
            if last < grid[innerloop][left] {
                last = grid[innerloop][left];

                let current_tree = (innerloop as u8, left as u8);
                if !counted_trees.contains(&current_tree) {
                    counted_trees.push(current_tree)
                }
            }

            innerloop += 1;
        }
    }

    //Watching from right
    for right in 0..size {
        let mut innerloop = 0;
        let mut last = grid[size - 1 - innerloop][right]; //x increments, y 0

        let current_tree = ((size - 1 - innerloop) as u8, right as u8);
        if !counted_trees.contains(&current_tree) {
            counted_trees.push(current_tree)
        }

        innerloop += 1;
        while innerloop < size {
            if last < grid[size - 1 - innerloop][right] {
                last = grid[size - 1 - innerloop][right];

                let current_tree = ((size - 1 - innerloop) as u8, right as u8);
                if !counted_trees.contains(&current_tree) {
                    counted_trees.push(current_tree)
                }
            }
            innerloop += 1;
        }
    }
}

fn create_grid(input: &String) -> Vec<Vec<u8>> {
    let width = input.lines().count();
    let height = width; //For posterity sake

    let mut array: Vec<Vec<u8>> = vec![vec![0; width]; height];

    for line in input.lines().into_iter().enumerate() {
        let y = line.0; //y coord is the line we're on. [0,0] is top left

        for tree in line.1.chars().enumerate() {
            let x = tree.0;
            let element = tree.1.to_digit(10).expect("cant parse :(") as u8;
            array[x][y] = element; //Looks weird printed, but makes sense when accessing imo
        }
    }

    array
}

#[cfg(test)]
mod tests {
    use crate::days::day8::{count_full, create_grid};

    use super::day_8_b;

    fn input() -> String {
        "30373
25512
65332
33549
35390"
            .to_string()
    }

    fn input2() -> String {
        "000000
009400
010203
009400
000000
000000"
            .to_string()
    }

    #[test]
    fn test_grid() {
        let input = input();
        let grid = create_grid(&input);

        assert_eq!(grid[0][0], 3);
        assert_eq!(grid[1][1], 5);
        assert_eq!(grid[3][1], 1);
        assert_eq!(grid[3][4], 9);
        assert_eq!(grid[4][4], 0);
    }

    #[test]
    fn test_day_a() {
        let input = input();

        let mut grid = create_grid(&input);
        let visible = count_full(&mut grid);

        assert_eq!(visible, 21);
    }

    #[test]
    fn test_weird_input() {
        let input = input2();
        let mut grid = create_grid(&input);
        let visible = count_full(&mut grid);
        assert_eq!(visible, 26);
    }

    #[test]
    fn test_day_b() {
        let input = input();

        let max_score = day_8_b(&input);
        assert_eq!(max_score, 8);
    }
}
