use regex::Regex;

#[must_use]
pub fn solve(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((?P<term_1>\d{1,3}),(?P<term_2>\d{1,3})\)").unwrap();

    let mut operations: Vec<(i32, i32)> = vec![];
    for (_, [term_1, term_2]) in regex.captures_iter(input).map(|c| c.extract()) {
        operations.push((term_1.parse().unwrap(), term_2.parse().unwrap()));
    }

    operations.iter().map(|(t1, t2)| t1 * t2).sum()
}
