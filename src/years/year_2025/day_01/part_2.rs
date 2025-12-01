const DIAL_START: i32 = 50;
const DIAL_MAX_VALUE: i32 = 99;

#[must_use]
pub fn solve(input: &str) -> i32 {
    let mut dial: i32 = DIAL_START;
    let range = DIAL_MAX_VALUE + 1;
    let mut zero_crossings = 0;
    let rotations = parse(input.trim());

    for delta in rotations {
        let crossings = if delta >= 0 {
            (dial + delta) / range
        } else {
            ((range - dial) % range - delta) / range
        };
        zero_crossings += crossings;
        dial = (dial + delta).rem_euclid(range);
    }

    zero_crossings
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let distance: i32 = line[1..].parse().unwrap();
            match direction {
                'L' => -distance,
                'R' => distance,
                _ => panic!("Invalid direction: {direction}"),
            }
        })
        .collect()
}
