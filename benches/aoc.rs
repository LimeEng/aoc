use aoc::{
    PuzzleId,
    storage::{self},
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::env;

fn bench_puzzle(c: &mut Criterion, year: u32, day: u32, part: u32) {
    let puzzle_id = PuzzleId::new(year, day, part);
    let password = env::var("AOC_KEY").unwrap();
    let puzzle = storage::get(&puzzle_id, &password).unwrap();

    let input = &puzzle.inputs.first().unwrap().input;
    c.bench_with_input(
        BenchmarkId::new("aoc", format!("{year}-{day:02}-{part}")),
        &input,
        |b, input| {
            b.iter(|| aoc::solve(year, day, part, input));
        },
    );
}

fn benchmark_puzzles(c: &mut Criterion) {
    generated_benchmarks(c);
}

include!(concat!(env!("OUT_DIR"), "/benchmarks.rs"));

criterion_group!(benches, benchmark_puzzles);
criterion_main!(benches);
