use std::collections::HashMap;
use std::fs;

fn find_path_to_out(map: &HashMap<String, Vec<String>>, from: &str) -> u64 {
    let list: Vec<String> = map.get(from).unwrap().to_vec();
    let total: u64 = list
        .iter()
        .map(|x| {
            if x == "out" {
                1
            } else {
                find_path_to_out(map, &x)
            }
        })
        .sum();
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
    let from: &str = "you";
    let number_of_paths: u64 = find_path_to_out(&map, from);
    println!("{}", number_of_paths)
}
