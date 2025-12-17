use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn read_ranges(path: &str) -> Vec<Range> {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let mut ranges = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split('-').collect();

        if parts.len() != 2 {
            break;
        }
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();

        ranges.push(Range { start, end });
    }
    ranges
}
fn main() {
    let path = "inputs/day05.txt";
    let mut ranges = read_ranges(path);
    ranges.sort_by_key(|r| r.start);
    let mut merged: Vec<(i64, i64)> = Vec::new();

    for range in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if range.start <= *last_end + 1 {
                *last_end = (*last_end).max(range.end);
                continue;
            }
        }
        merged.push((range.start, range.end));
    }
    let total: i128 = merged
        .iter()
        .map(|(s, e)| (*e as i128 - *s as i128 + 1))
        .sum();

    println!("{}", total);
}
