use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(&self, value: i64) -> bool {
        self.start <= value && value <= self.end
    }
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

fn read_values(path: &str) -> Vec<i64> {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let mut after_blank = false;
    let mut values = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            after_blank = true;
            continue;
        }
        if after_blank {
            let value = line.trim().parse().unwrap();
            values.push(value);
        }
    }
    values
}
fn main() {
    let mut ctr = 0;
    let path = "inputs/day05.txt";
    let ranges = read_ranges(path);
    let values = read_values(path);

    for value in values {
        if ranges.iter().any(|r| r.contains(value)) {
            ctr += 1;
        }
    }
    println!("{}", ctr);
}
