#[must_use]
pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    for x in 1..grid.len() - 1 {
        for y in 1..grid[x].len() - 1 {
            if check_position(&grid, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn check_position(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if grid[x][y] != 'A' {
        return false;
    }
    let mut part_1 = false;
    let mut part_2 = false;
    if x + 1 < grid.len() && y + 1 < grid[0].len() {
        // Top right to bottom left
        if grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S' {
            part_1 = true;
        }
        if grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M' {
            part_1 = true;
        }

        // Top left to bottom right
        if grid[x - 1][y + 1] == 'M' && grid[x + 1][y - 1] == 'S' {
            part_2 = true;
        }
        if grid[x - 1][y + 1] == 'S' && grid[x + 1][y - 1] == 'M' {
            part_2 = true;
        }
    }

    part_1 && part_2
}
