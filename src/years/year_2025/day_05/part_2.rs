use std::ops::RangeInclusive;

#[must_use]
pub fn solve(input: &str) -> u64 {
    let mut ranges = parse(input);
    ranges.sort_by_key(|r| *r.start());

    ranges
        .into_iter()
        .fold(Vec::new(), |mut merged, range| {
            match merged
                .last_mut()
                .and_then(|last| merge_ranges(last, &range))
            {
                Some(new_range) => *merged.last_mut().unwrap() = new_range,
                None => merged.push(range),
            }
            merged
        })
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

fn merge_ranges(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    let overlaps = a.start() <= b.end() && b.start() <= a.end();
    let adjacent = a.end() + 1 == *b.start() || b.end() + 1 == *a.start();

    if overlaps || adjacent {
        Some(*a.start().min(b.start())..=*a.end().max(b.end()))
    } else {
        None
    }
}

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect()
}
