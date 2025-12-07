struct JunctionBox {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl JunctionBox {
    fn distance_squared(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = vec![0; n];
        for i in 0..n {
            sizes[self.find(i)] += 1;
        }
        sizes.into_iter().filter(|&s| s > 0).collect()
    }
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let boxes = parse(input);
    let mut distances = compute_distances(&boxes);
    distances.sort_unstable();

    let mut uf = UnionFind::new(boxes.len());
    for &(_, i, j) in distances.iter().take(1000) {
        uf.union(i, j);
    }

    let mut sizes = uf.circuit_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product::<usize>() as u64
}

fn compute_distances(boxes: &[JunctionBox]) -> Vec<(u64, usize, usize)> {
    let mut distances = Vec::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            distances.push((boxes[i].distance_squared(&boxes[j]), i, j));
        }
    }
    distances
}

fn parse(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|s| s.parse().unwrap());
            JunctionBox {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
                z: coords.next().unwrap(),
            }
        })
        .collect()
}
