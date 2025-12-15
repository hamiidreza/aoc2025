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
        let mut line_max: u64 = 0;
        let mut idx = 0;
        for i in 0..12 {
            let truncated = &line[idx..line.len() - 11 + i];
            let (max, new_idx) = max_digit_index(truncated);
            idx = new_idx + idx + 1;
            let exp = 11_u32
            .checked_sub(i.try_into().unwrap())
            .expect("i too big");
            line_max += 10_u64.pow(exp) * max as u64;

        }
        println!("{}", line_max);
        sum += line_max;
    }

    println!("{}", sum);
}
