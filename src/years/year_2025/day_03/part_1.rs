#[must_use]
pub fn solve(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|bank: &Vec<u64>| max_joltage(bank))
        .sum()
}

fn max_joltage(bank: &[u64]) -> u64 {
    (0..bank.len())
        .flat_map(|i| ((i + 1)..bank.len()).map(move |j| (i, j)))
        .map(|(i, j)| bank[i] * 10 + bank[j])
        .max()
        .unwrap()
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
