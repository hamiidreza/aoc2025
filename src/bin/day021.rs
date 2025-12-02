use std::fs;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_range(s: &str) -> Range {
    let (a, b) = s.split_once('-').expect("invalid range");
    Range {
        start: a.trim().parse().unwrap(),
        end: b.trim().parse().unwrap(),
    }
}

fn split_number(n: u64) -> (u64, u64) {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return (0, 1);
    }
    let mid = s.len() / 2;

    let left = &s[..mid];
    let right = &s[mid..];

    (left.parse().unwrap(), right.parse().unwrap())
}

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read input file");
    let ranges: Vec<Range> = input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_range)
        .collect();

    let mut ctr = 0;
    for range in ranges {
        let start = range.start;
        let end = range.end;
        for num in start..=end {
            let (left, right) = split_number(num);
            if left == right {
                ctr += num;
            }
        }
    }
    println!("{}", ctr);
}
