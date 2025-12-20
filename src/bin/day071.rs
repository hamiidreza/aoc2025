use std::{collections::HashSet, fs};

fn beam(grid: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) {
    if x >= grid.len() || y >= grid[0].len() {
        return;
    }
    match grid[x][y] {
        '.' => {
            beam(grid, x + 1, y, visited);
        }
        '^' => {
            if visited.insert((x, y)) {
                beam(grid, x + 1, y - 1, visited);
                beam(grid, x + 1, y + 1, visited);
            }
        }
        _ => {}
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let start_y = grid[0].iter().position(|&c| c == 'S').unwrap();
    beam(&grid, 1, start_y, &mut visited_positions);
    println!("{:?}", visited_positions.len());
}
