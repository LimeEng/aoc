use crate::{PuzzleId, discovery::discover_tests};

pub fn generate() -> String {
    println!("cargo:rerun-if-changed=puzzles.enc");
    let tests = discover_tests();
    generate_tests(&tests)
}

fn generate_tests(tests: &[PuzzleId]) -> String {
    tests
        .iter()
        .map(|puzzle| {
            format!(
                "#[test]\nfn test_{}_{:02}_{:02}() {{ super::run_test({}, {}, {}); }}",
                puzzle.year, puzzle.day, puzzle.part, puzzle.year, puzzle.day, puzzle.part
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
