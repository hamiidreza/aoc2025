use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Plus,
    Mult,
}

struct Instance {
    val1: i64,
    val2: i64,
    val3: i64,
    val4: i64,
    op: Op,
}

impl Instance {
    fn calculate(&self) -> i64 {
        match self.op {
            Op::Plus => self.val1 + self.val2 + self.val3 + self.val4,
            Op::Mult => self.val1 * self.val2 * self.val3 * self.val4,
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();

    let vals1: Vec<i64> = lines
        .get(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();

    let vals2: Vec<i64> = lines
        .get(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();

    let vals3: Vec<i64> = lines
        .get(2)
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();

    let vals4: Vec<i64> = lines
        .get(3)
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();

    let ops: Vec<Op> = lines
        .get(4)
        .unwrap()
        .split_whitespace()
        .filter_map(|v| match v {
            "+" => Some(Op::Plus),
            "*" => Some(Op::Mult),
            _ => None,
        })
        .collect();

    let mut sum: i64 = 0;
    for i in 0..vals1.len() {
        let instance = Instance {
            val1: vals1[i],
            val2: vals2[i],
            val3: vals3[i],
            val4: vals4[i],
            op: ops[i],
        };
        sum += instance.calculate();
    }
    println!("{sum}")
}
