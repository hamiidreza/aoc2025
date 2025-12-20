use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn distance(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).isqrt()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        if self.size[px] < self.size[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day08.txt").expect("Failed to read input file");

    let positions: Vec<Vec3> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            Vec3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let n = positions.len();
    let mut uf = UnionFind::new(n);

    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = positions[i].distance(&positions[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_by_key(|&(dist, _, _)| dist);

    for &(_, i, j) in pairs.iter().take(1000) {
        uf.union(i, j);
    }

    let mut set_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *set_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = set_sizes.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending

    let result: usize = sizes.iter().take(3).product();
    println!("{}", result);
}
