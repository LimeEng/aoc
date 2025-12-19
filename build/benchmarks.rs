use crate::{PuzzleId, discovery::discover_benchmarks};

pub fn generate() -> String {
    println!("cargo:rerun-if-changed=puzzles.enc");
    let benchmarks = discover_benchmarks();
    generate_benchmarks(&benchmarks)
}

fn generate_benchmarks(benchmarks: &[PuzzleId]) -> String {
    let calls = benchmarks
        .iter()
        .map(|puzzle| {
            format!(
                "    bench_puzzle(c, {}, {}, {});",
                puzzle.year, puzzle.day, puzzle.part
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("fn generated_benchmarks(c: &mut Criterion) {{\n{calls}\n}}",)
}
