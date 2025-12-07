use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '^' => Cell::Splitter,
            'S' => Cell::Beam,
            _ => panic!("Unknown character '{value}'"),
        }
    }
}

type Manifold = Vec<Vec<Cell>>;

#[must_use]
pub fn solve(input: &str) -> u64 {
    simulate(&parse(input))
}

fn parse(input: &str) -> Manifold {
    input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect()
}

fn simulate(manifold: &Manifold) -> u64 {
    let mut total_timelines = 0;
    let mut queue = VecDeque::new();
    let mut path_count: HashMap<(usize, usize), u64> = HashMap::new();

    let start_col = manifold[0]
        .iter()
        .position(|&cell| cell == Cell::Beam)
        .unwrap();

    queue.push_back((0, start_col));
    path_count.insert((0, start_col), 1);

    while let Some((row, col)) = queue.pop_front() {
        let count = *path_count.get(&(row, col)).unwrap();

        let next_row = row + 1;
        if next_row >= manifold.len() {
            total_timelines += count;
            continue;
        }

        let cell = manifold[next_row][col];
        let next_positions = match cell {
            Cell::Splitter => {
                let mut positions = Vec::new();
                if col > 0 {
                    positions.push((next_row, col - 1));
                }
                if col < manifold[next_row].len() - 1 {
                    positions.push((next_row, col + 1));
                }
                positions
            }
            Cell::Empty | Cell::Beam => vec![(next_row, col)],
        };

        for pos in next_positions {
            let entry = path_count.entry(pos).or_insert(0);
            if *entry == 0 {
                queue.push_back(pos);
            }
            *entry += count;
        }
    }

    total_timelines
}
