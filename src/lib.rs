#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod cli;
pub mod storage;
mod years;

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
        assert!(part == 1 || part == 2, "Invalid part");
        assert!((1..25).contains(&day), "Invalid day");
        assert!(year >= 2015, "Invalid year");

        Self { year, day, part }
    }
}
