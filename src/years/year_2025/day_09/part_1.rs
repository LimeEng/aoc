struct RedTile {
    x: i64,
    y: i64,
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let tiles = parse(input);

    let mut max_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let area = rectangle_area(&tiles[i], &tiles[j]);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn rectangle_area(t1: &RedTile, t2: &RedTile) -> u64 {
    let width = t1.x.abs_diff(t2.x) + 1;
    let height = t1.y.abs_diff(t2.y) + 1;
    width * height
}

fn parse(input: &str) -> Vec<RedTile> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            RedTile { x, y }
        })
        .collect()
}
