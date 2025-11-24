use std::collections::HashSet;

#[must_use]
pub fn solve(input: &str) -> usize {
    let polymer: Vec<_> = input.trim().chars().collect();
    let available_units: HashSet<_> = polymer.iter().filter(|unit| unit.is_lowercase()).collect();
    available_units
        .iter()
        .map(|unit| {
            let mut copy = polymer.clone();
            copy.retain(|u| u.to_ascii_lowercase() != **unit);
            copy
        })
        .map(|polymer| collapse_polymer(&polymer))
        .min()
        .unwrap()
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
