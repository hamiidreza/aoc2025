use std::fs;

type Grid = Vec<Vec<char>>;

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> i32 {
    let mut ctr = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && nx < grid.len() as isize && ny < grid[0].len() as isize {
            if grid[nx as usize][ny as usize] == '@' {
                ctr += 1;
            }
        }
    }
    ctr
}

fn run_until_stable(grid: &Grid) -> (Grid, bool) {
    let mut new_grid = grid.clone();
    let mut changed = false;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '@' {
                if count_neighbors(&grid, x, y) < 4 {
                    new_grid[x][y] = '.';
                    changed = true;
                }
            }
        }
    }
    (new_grid, changed)
}

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read input file");
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut papers_total = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '@' {
                papers_total += 1;
            }
        }
    }

    loop {
        let (next_grid, changed) = run_until_stable(&grid);
        grid = next_grid;
        if !changed {
            break;
        }
    }

    let mut papers_left = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '@' {
                papers_left += 1;
            }
        }
    }
    println!("{}", papers_total - papers_left)
}
