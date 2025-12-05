use std::ops::RangeInclusive;

#[must_use]
pub fn solve(input: &str) -> usize {
    let (ranges, numbers) = parse(input);

    numbers
        .iter()
        .filter(|num| ranges.iter().any(|range| range.contains(num)))
        .count()
}

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges_str, numbers_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let numbers = numbers_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, numbers)
}
