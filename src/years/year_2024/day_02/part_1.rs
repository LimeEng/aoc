#[must_use]
pub fn solve(input: &str) -> i32 {
    let reports: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();

    reports
        .iter()
        .map(|report| is_safe(report))
        .map(i32::from)
        .sum()
}

fn is_safe(report: &[i32]) -> bool {
    let first_window = report.iter().take(report.len() - 1);
    let second_window = report.iter().skip(1);

    let is_ascending = std::iter::zip(first_window.clone(), second_window.clone())
        .all(|(&a, &b)| b - a >= 1 && b - a <= 3);
    let is_descending =
        std::iter::zip(first_window, second_window).all(|(&a, &b)| a - b >= 1 && a - b <= 3);

    is_ascending || is_descending
}
