use std::fs;

fn count_neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
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

fn main() {
    let mut sum = 0;
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '@' {
                if count_neighbors(&grid, x, y) < 4 {
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum)
}
