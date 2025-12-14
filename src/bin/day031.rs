use std::fs;

fn max_digit_index(s: &str) -> (u32, usize) {
    let mut max_digit = None;
    let mut max_index = None;

    for (i, c) in s.char_indices() {
        if let Some(d) = c.to_digit(10) {
            if max_digit.map_or(true, |m| d > m) {
                max_digit = Some(d);
                max_index = Some(i);
            }
        }
    }
    (max_digit.unwrap(), max_index.unwrap())
}

fn main() {
    let mut sum = 0;
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read input file");
    for line in input.lines() {
        let (max, idx) = max_digit_index(line);
        let last_char_index = line.char_indices().last().map(|(i, _)| i);
        let (max, idx) = if Some(idx) == last_char_index {
            let truncated = &line[..idx];
            max_digit_index(truncated)
        } else {
            (max, idx)
        };

        let (second_max, _) = max_digit_index(&line[idx + 1..]);

        let line_max: u32 = max * 10 + second_max;
        sum += line_max;
    }

    println!("{}", sum);
}
