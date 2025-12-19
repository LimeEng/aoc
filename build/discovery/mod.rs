use crate::PuzzleId;

mod benchmarks;
mod docs;
mod solvers;
mod tests;

pub fn discover_solvers() -> Vec<PuzzleId> {
    solvers::discover()
}

pub fn discover_tests() -> Vec<PuzzleId> {
    tests::discover()
}

pub fn discover_benchmarks() -> Vec<PuzzleId> {
    benchmarks::discover()
}

pub fn discover_docs() -> Vec<PuzzleId> {
    docs::discover()
}
