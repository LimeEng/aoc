#[allow(dead_code)]
struct Machine {
    buttons: Vec<Vec<u64>>,
    joltage: Vec<u64>,
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let _machines = parse(input);

    9
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (_lights, line) = line.split_once(' ').unwrap();

            let (buttons, joltage) = line.rsplit_once(' ').unwrap();
            let buttons = buttons
                .split_whitespace()
                .map(|button| {
                    button
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltage = joltage
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            Machine { buttons, joltage }
        })
        .collect()
}
