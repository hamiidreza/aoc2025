use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = lines.len();
    let cols = lines.iter().map(|l| l.len()).max().unwrap();

    let mut grid = lines;
    for row in &mut grid {
        row.resize(cols, ' ');
    }

    let mut grand_total: u128 = 0;
    let mut c: isize = cols as isize - 1;

    while c >= 0 {
        if grid.iter().all(|row| row[c as usize] == ' ') {
            c -= 1;
            continue;
        }

        let mut problem_cols = Vec::new();
        while c >= 0 && !grid.iter().all(|row| row[c as usize] == ' ') {
            problem_cols.push(c as usize);
            c -= 1;
        }

        let op = problem_cols
            .iter()
            .map(|&col| grid[rows - 1][col])
            .find(|&ch| ch == '+' || ch == '*')
            .expect("Operator not found");

        let mut numbers = Vec::new();

        for &col in &problem_cols {
            let mut digits = String::new();
            for r in 0..rows - 1 {
                let ch = grid[r][col];
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }
            if !digits.is_empty() {
                numbers.push(digits.parse::<u128>().unwrap());
            }
        }

        let value = match op {
            '+' => numbers.iter().sum::<u128>(),
            '*' => numbers.iter().product::<u128>(),
            _ => panic!("Invalid operator: {}", op),
        };

        grand_total += value;
    }

    println!("Grand total: {}", grand_total);
}
