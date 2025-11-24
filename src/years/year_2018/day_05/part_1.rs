#[must_use]
pub fn solve(input: &str) -> usize {
    let polymer: Vec<_> = input.trim().chars().collect();
    collapse_polymer(&polymer)
}

fn collapse_polymer(polymer: &[char]) -> usize {
    polymer
        .iter()
        .fold(Vec::new(), |mut stack, &unit| {
            if stack.last().is_some_and(|&last| does_react(last, unit)) {
                stack.pop();
            } else {
                stack.push(unit);
            }
            stack
        })
        .len()
}

fn does_react(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && a.is_uppercase() == b.is_lowercase()
}
