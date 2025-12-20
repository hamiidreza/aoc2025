use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let start_y: usize = grid[0].iter().position(|&c| c == 'S').unwrap();

    let mut total_reached = vec![vec![0u128; cols]; rows];
    total_reached[0][start_y] = 1;

    for x in 0..rows - 1 {
        for y in 0..cols {
            //let count = totalReached[x][y];
            if total_reached[x][y] == 0 {
                continue;
            }

            match grid[x][y] {
                '^' => {
                    if y > 0 {
                        total_reached[x + 1][y - 1] += total_reached[x][y];
                    }
                    if y + 1 < cols {
                        total_reached[x + 1][y + 1] += total_reached[x][y];
                    }
                }
                _ => {
                    total_reached[x + 1][y] += total_reached[x][y];
                }
            }
        }
    }
    let total: u128 = total_reached[rows - 1].iter().sum();
    println!("{}", total);
}
