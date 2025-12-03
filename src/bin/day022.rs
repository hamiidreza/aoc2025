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

fn split_number_into_chunks(n: u64, chunk_size: usize) -> Vec<u64> {
    let s = n.to_string();
    let mut v = Vec::new();
    for i in 0..(s.len() / chunk_size) {
        v.push(s[i * chunk_size..(i + 1) * chunk_size].parse().unwrap());
    }
    v
}

fn all_equal(v: &[u64]) -> bool {
    if let Some(first) = v.first() {
        v.iter().all(|&x| x == *first)
    } else {
        false
    }
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
            let s = num.to_string();
            for i in 1..=s.len() / 2 {
                if s.len() % i == 0 {
                    let v = split_number_into_chunks(num, i);
                    if all_equal(&v) {
                        ctr += num;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ctr);
}
