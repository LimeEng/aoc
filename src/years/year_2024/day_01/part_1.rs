#[must_use]
pub fn solve(input: &str) -> i32 {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut values = line.split_whitespace();
        a.push(values.next().unwrap().parse::<i32>().unwrap());
        b.push(values.next().unwrap().parse::<i32>().unwrap());
    }
    // They are only integers, a stable sort will not produce any observable difference
    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}
