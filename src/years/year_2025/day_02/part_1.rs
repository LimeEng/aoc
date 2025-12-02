use std::ops::RangeInclusive;

#[must_use]
pub fn solve(input: &str) -> u64 {
    let ranges = parse(input.trim());

    ranges
        .iter()
        .flat_map(Clone::clone)
        .filter(is_repeated_twice)
        .sum()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_repeated_twice(n: &u64) -> bool {
    let text = n.to_string();
    let len = text.len();
    len.is_multiple_of(2) && text[..len / 2] == text[len / 2..]
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
