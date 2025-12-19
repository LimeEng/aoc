#![doc(html_logo_url = "https://cdn.github.emileng.se/repo/aoc/festive_ferris.svg")]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod api;
pub mod cli;
pub mod storage;
pub mod years;

include!(concat!(env!("OUT_DIR"), "/solvers.rs"));

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PuzzleId {
    year: u32,
    day: u32,
    part: u32,
}

impl PuzzleId {
    #[must_use]
    pub fn new(year: u32, day: u32, part: u32) -> Self {
        assert!(year >= 2015, "Invalid year");
        if year >= 2025 {
            assert!((1..=12).contains(&day), "Invalid day");
        } else {
            assert!((1..=25).contains(&day), "Invalid day");
        }
        assert!(part == 1 || part == 2, "Invalid part");

        Self { year, day, part }
    }
}
