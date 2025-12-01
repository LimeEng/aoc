const DIAL_START: i32 = 50;
const DIAL_MAX_VALUE: i32 = 99;

#[must_use]
pub fn solve(input: &str) -> usize {
    let mut dial = DIAL_START;
    let range = DIAL_MAX_VALUE + 1;
    let mut number_of_zeros = 0;
    let rotations = parse(input.trim());

    for delta in rotations {
        dial += delta;
        if dial % range == 0 {
            number_of_zeros += 1;
        }
    }

    number_of_zeros
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
