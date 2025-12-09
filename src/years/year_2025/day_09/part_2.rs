struct RedTile {
    x: i64,
    y: i64,
}

struct Rectangle {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Rectangle {
    fn new(t1: &RedTile, t2: &RedTile) -> Self {
        Rectangle {
            min_x: t1.x.min(t2.x),
            min_y: t1.y.min(t2.y),
            max_x: t1.x.max(t2.x),
            max_y: t1.y.max(t2.y),
        }
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        let x_overlap = self.min_x <= other.max_x && self.max_x >= other.min_x;
        let y_overlap = self.min_y <= other.max_y && self.max_y >= other.min_y;
        x_overlap && y_overlap
    }

    fn inset(&self, amount: i64) -> Self {
        Rectangle {
            min_x: self.min_x + amount,
            min_y: self.min_y + amount,
            max_x: self.max_x - amount,
            max_y: self.max_y - amount,
        }
    }

    fn area(&self) -> u64 {
        let width = self.min_x.abs_diff(self.max_x) + 1;
        let height = self.min_y.abs_diff(self.max_y) + 1;
        width * height
    }

    fn center(&self) -> (i64, i64) {
        (
            i64::midpoint(self.min_x, self.max_x),
            i64::midpoint(self.min_y, self.max_y),
        )
    }
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let tiles = parse(input);

    let mut edges = Vec::with_capacity(tiles.len());
    for i in 0..tiles.len() {
        let next = (i + 1) % tiles.len();
        edges.push(Rectangle::new(&tiles[i], &tiles[next]));
    }

    let mut max_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            if is_valid_rectangle(&tiles[i], &tiles[j], &tiles, &edges) {
                let rect = Rectangle::new(&tiles[i], &tiles[j]);
                max_area = max_area.max(rect.area());
            }
        }
    }

    max_area
}

fn is_valid_rectangle(t1: &RedTile, t2: &RedTile, tiles: &[RedTile], edges: &[Rectangle]) -> bool {
    let rect = Rectangle::new(t1, t2);
    let inner = rect.inset(1);

    if edges.iter().any(|e| inner.intersects(e)) {
        return false;
    }

    let (center_x, center_y) = inner.center();
    is_inside_polygon(center_x, center_y, tiles)
}

fn is_inside_polygon(x: i64, y: i64, tiles: &[RedTile]) -> bool {
    let mut crossings = 0;

    for i in 0..tiles.len() {
        let next = (i + 1) % tiles.len();
        let t1 = &tiles[i];
        let t2 = &tiles[next];

        if (t1.y <= y && y < t2.y) || (t2.y <= y && y < t1.y) {
            let x_intersect = t1.x + (y - t1.y) * (t2.x - t1.x) / (t2.y - t1.y);
            if x < x_intersect {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
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
