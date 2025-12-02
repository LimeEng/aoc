use std::ops::RangeInclusive;

#[must_use]
pub fn solve(input: &str) -> u64 {
    let ranges = parse(input.trim());

    ranges
        .iter()
        .flat_map(Clone::clone)
        .filter(is_repeated_at_least_twice)
        .sum()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_repeated_at_least_twice(n: &u64) -> bool {
    let text = n.to_string();
    let doubled = text.repeat(2);
    let len = doubled.len();
    doubled[1..len - 1].contains(&text)
}

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .and_then(|(start, end)| Some(start.parse().ok()?..=end.parse().ok()?))
                .unwrap()
        })
        .collect()
}
