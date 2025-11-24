use std::collections::HashMap;

#[must_use]
pub fn solve(input: &str) -> i32 {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut values = line.split_whitespace();
        a.push(values.next().unwrap().parse::<i32>().unwrap());
        b.push(values.next().unwrap().parse::<i32>().unwrap());
    }

    let mut count = HashMap::new();
    for value in &b {
        count.entry(value).and_modify(|e| *e += 1).or_insert(1);
    }

    a.iter()
        .map(|value| value * count.get(value).unwrap_or(&0))
        .sum()
}
