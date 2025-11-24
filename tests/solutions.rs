use aoc::{
    PuzzleId,
    storage::{self},
};
use std::env;

fn run_test(year: u32, day: u32, part: u32) {
    let test_id = PuzzleId::new(year, day, part);
    let password = env::var("AOC_KEY").unwrap();
    let puzzle = storage::get(&test_id, &password).unwrap();
    for test in puzzle.tests {
        assert_eq!(
            aoc::solve(year, day, part, &test.input),
            Some(test.expected)
        );
    }
}

mod generated_tests {
    include!(concat!(env!("OUT_DIR"), "/tests.rs"));
}
