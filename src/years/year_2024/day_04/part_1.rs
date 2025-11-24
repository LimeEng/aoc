#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let directions = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            for (dx, dy) in &directions {
                if check_position(&grid, x, y, *dx, *dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_position(grid: &[Vec<char>], x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let target = "XMAS";

    for (i, chr) in target.chars().enumerate() {
        let x: isize = x as isize + (dx * i as isize);
        let y: isize = y as isize + (dy * i as isize);

        if x < 0 || y < 0 {
            return false;
        }
        let grid_chr = grid.get(x as usize).and_then(|slice| slice.get(y as usize));
        if let Some(grid_chr) = grid_chr {
            if chr != *grid_chr {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
