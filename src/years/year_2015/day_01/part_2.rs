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
pub fn solve(input: &str) -> u64 {
    input
        .chars()
        .map(Direction::from)
        .map(Direction::delta)
        .scan(0, |floor, delta| {
            *floor += delta;
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .map(|pos| pos + 1)
        .unwrap()
        .try_into()
        .unwrap()
}
