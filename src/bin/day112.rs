use std::collections::HashMap;
use std::fs;

fn count_paths(
    map: &HashMap<String, Vec<String>>,
    node: &str,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let state = (node.to_string(), visited_dac, visited_fft);
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    let visited_dac = visited_dac || node == "dac";
    let visited_fft = visited_fft || node == "fft";

    let result = if node == "out" {
        if visited_dac && visited_fft { 1 } else { 0 }
    } else {
        let mut total = 0;
        if let Some(children) = map.get(node) {
            for child in children {
                total += count_paths(map, child, visited_dac, visited_fft, memo);
            }
        }
        total
    };

    memo.insert(state, result);
    result
}

fn main() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Failed to read input file");
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once(':').unwrap();
        map.insert(
            left.trim().to_string(),
            right.trim().split_whitespace().map(String::from).collect(),
        );
    }

    let mut memo = HashMap::new();
    let total_paths = count_paths(&map, "svr", false, false, &mut memo);
    println!("{}", total_paths);
}
