use std::fs;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

fn max_area(corner_one: &Vec2, corner_two: &Vec2) -> i64 {
    let dx = (corner_one.x - corner_two.x).abs() + 1;
    let dy = (corner_one.y - corner_two.y).abs() + 1;
    dx * dy
}

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").expect("Failed to read input file");

    let positions: Vec<Vec2> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            Vec2 {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    let mut output = 0;
    for i in &positions {
        for j in &positions {
            output = output.max(max_area(i, j));
        }
    }
    println!("{}", output);
}
