use std::collections::HashMap;
use std::fs;

fn count_paths_with_dac_fft(
    map: &HashMap<String, Vec<String>>,
    node: &str,
    visited_dac: bool,
    visited_fft: bool,
) -> usize {
    let visited_dac = visited_dac || node == "dac";
    let visited_fft = visited_fft || node == "fft";

    if node == "out" {
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    let mut total = 0;
    if let Some(children) = map.get(node) {
        for child in children {
            total += count_paths_with_dac_fft(map, child, visited_dac, visited_fft);
        }
    }
    total
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
    let total_paths = count_paths_with_dac_fft(&map, "svr", false, false);
    println!("{}", total_paths);
}
