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
                if pos == 0 {
                    pos -= x;
                    while pos < 0 {
                        pos += 100;
                        ctr += 1;
                    }
                    ctr -= 1;
                } else {
                    pos -= x;
                    while pos < 0 {
                        pos += 100;
                        ctr += 1;
                    }
                    if pos == 0 {
                        ctr += 1;
                    }
                }
            }
            'R' => {
                pos += x;
                while pos >= 100 {
                    pos -= 100;
                    ctr += 1;
                }
            }
            _ => {
                panic!("Invalid direction: {}", dir);
            }
        }
    }
    println!("{}", ctr);
}
