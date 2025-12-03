#[must_use]
pub fn solve(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|bank: &Vec<u64>| max_joltage(bank))
        .sum()
}

fn max_joltage(bank: &[u64]) -> u64 {
    let k = 12;
    let to_remove = bank.len() - k;
    let mut stack = Vec::new();
    let mut removals_left = to_remove;

    for &digit in bank {
        while !stack.is_empty() && removals_left > 0 && *stack.last().unwrap() < digit {
            stack.pop();
            removals_left -= 1;
        }
        stack.push(digit);
    }

    stack.truncate(k);

    stack.iter().fold(0u64, |acc, &digit| acc * 10 + digit)
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_string())
                .map(|digit| digit.parse().unwrap())
                .collect()
        })
        .collect()
}
