use aoc::input_storage::{self, EntryId};
use std::env;

fn run_test(year: u32, day: u32, part: u32) {
    let test_id = EntryId::new(year, day, part);
    let password = env::var("AOC_KEY").unwrap();
    let test_cases = input_storage::read(&test_id, &password).unwrap();
    for test_case in test_cases {
        assert_eq!(
            aoc::solve(year, day, part, &test_case.input),
            Some(test_case.expected)
        );
    }
}

mod generated_tests {
    include!(concat!(env!("OUT_DIR"), "/tests.rs"));
}
