enum Direction {
    Up,
    Down,
}

impl Direction {
    fn delta(self) -> i64 {
        match self {
            Self::Up => 1,
            Self::Down => -1,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::Up,
            ')' => Self::Down,
            _ => panic!("Unknown character '{value}'"),
        }
    }
}

#[must_use]
pub fn solve(input: &str) -> i64 {
    input
        .chars()
        .map(Direction::from)
        .map(Direction::delta)
        .sum()
}
