use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read input file");

    let mut ctr: u32 = 0;
    let mut pos: i32 = 50;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let x = line[1..].parse::<i32>().unwrap();
        match dir {
            'L' => {
                pos -= x;
                pos = wrap(pos);
            }
            'R' => {
                pos += x;
                pos = wrap(pos);
            }
            _ => {
                panic!("Invalid direction: {}", dir);
            }
        }
        if pos == 0 {
            ctr += 1;
        }
    }
    println!("{}", ctr);
}

fn wrap(pos: i32) -> i32 {
    ((pos % 100) + 100) % 100
}
