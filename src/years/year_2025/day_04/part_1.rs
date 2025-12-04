type Grid = Vec<Vec<Cell>>;

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '@' => Cell::Paper,
            '.' => Cell::Empty,
            chr => panic!("Unknown character '{chr}'"),
        }
    }
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let grid = parse(input);

    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == Cell::Paper {
                let neighbors = count_neighbors(&grid, x, y);
                if neighbors < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    #[rustfmt::skip]
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    neighbors
        .iter()
        .filter_map(|(dx, dy)| {
            let x = x.checked_add_signed(*dx)?;
            let y = y.checked_add_signed(*dy)?;
            if x < rows && y < cols {
                Some((x, y))
            } else {
                None
            }
        })
        .filter(|&(x, y)| grid[x][y] == Cell::Paper)
        .count()
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect()
}
