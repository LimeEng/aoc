use crate::{PuzzleId, discovery::discover_solvers};
use std::fmt::Write;

pub fn generate() -> String {
    println!("cargo:rerun-if-changed=src/years");
    let solutions = discover_solvers();
    generate_solvers(&solutions)
}

fn generate_solvers(solutions: &[PuzzleId]) -> String {
    let mut code = String::from(
        "\
#[must_use]
pub fn solve(year: u32, day: u32, part: u32, input: &str) -> Option<String> {
    match (year, day, part) {\n",
    );

    for solution in solutions {
        writeln!(
            code,
            "        ({}, {}, {}) => Some(format!(\"{{}}\", years::year_{}::day_{:02}::part_{}::solve(input))),",
            solution.year, solution.day, solution.part,
            solution.year, solution.day, solution.part
        ).unwrap();
    }

    code.push_str("        _ => None,\n    }\n}");
    code
}
