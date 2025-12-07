use std::collections::{HashSet, VecDeque};

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
    let mut splits = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let start_col = manifold[0]
        .iter()
        .position(|&cell| cell == Cell::Beam)
        .unwrap();

    queue.push_back((0, start_col));
    while let Some((row, col)) = queue.pop_front() {
        if !visited.insert((row, col)) {
            continue;
        }

        let next_row = row + 1;
        if next_row >= manifold.len() {
            continue;
        }

        let cell = manifold[next_row][col];
        match cell {
            Cell::Splitter => {
                splits += 1;

                if col > 0 {
                    queue.push_back((next_row, col - 1));
                }

                if col < manifold[next_row].len() - 1 {
                    queue.push_back((next_row, col + 1));
                }
            }
            Cell::Empty | Cell::Beam => {
                queue.push_back((next_row, col));
            }
        }
    }

    splits
}
