use std::{collections::HashSet, fs};

fn beam(grid: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) {
    for i in x..grid.len() - 1 {
        if grid[i + 1][y] == '^' && visited.insert((i + 1, y)) {
            visited.insert((i + 1, y));
            beam(&grid, i + 1, y - 1, visited);
            beam(&grid, i + 1, y + 1, visited);
            break;
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid[0].len() {
        if grid[0][y] == 'S' {
            beam(&grid, 1, y, &mut visited_positions);
            break;
        }
    }

    println!("{:?}", visited_positions.len());
}
