use crate::{PuzzleId, solve, storage};
use clap::{Arg, ArgMatches, Command, value_parser};
use inquire::Select;
use std::{env, time::Instant};

enum InputType<'a> {
    Test(&'a storage::TestCase),
    Input(&'a storage::PuzzleInput),
}

#[must_use]
pub fn command() -> Command {
    Command::new("solve")
        .arg(Arg::new("year").value_parser(value_parser!(u32)).index(1))
        .arg(Arg::new("day").value_parser(value_parser!(u32)).index(2))
        .arg(Arg::new("part").value_parser(value_parser!(u32)).index(3))
}

pub fn execute(matches: &ArgMatches) {
    if let Err(e) = try_execute(matches) {
        println!("Error: {e}");
    }
}

fn try_execute(matches: &ArgMatches) -> Result<(), String> {
    let password = env::var("AOC_KEY").map_err(|_| "AOC_KEY environment variable not set")?;

    let year = matches.get_one::<u32>("year").copied();
    let day = matches.get_one::<u32>("day").copied();
    let part = matches.get_one::<u32>("part").copied();

    // If all arguments provided, run directly
    if let (Some(y), Some(d), Some(p)) = (year, day, part) {
        let puzzle_id = PuzzleId::new(y, d, p);
        let puzzle = storage::get(&puzzle_id, &password)
            .map_err(|_| format!("Failed to load puzzle {y} day {d:02} part {p}"))?;
        return run_puzzle(&puzzle);
    }

    // Load and filter puzzles
    let puzzles = storage::get_all(&password).map_err(|_| "Failed to load puzzles")?;
    let filtered: Vec<_> = puzzles
        .iter()
        .filter(|p| year.is_none_or(|y| p.id.year == y))
        .filter(|p| day.is_none_or(|d| p.id.day == d))
        .filter(|p| part.is_none_or(|pt| p.id.part == pt))
        .collect();

    let puzzle = select_one(&filtered, format_puzzle_option)
        .ok_or("No puzzles match the specified criteria")?;

    run_puzzle(puzzle)
}

fn run_puzzle(puzzle: &storage::Puzzle) -> Result<(), String> {
    let mut options: Vec<(InputType, String)> = Vec::new();

    for test in &puzzle.tests {
        options.push((InputType::Test(test), format!("Test {}", test.id)));
    }

    for input in &puzzle.inputs {
        options.push((InputType::Input(input), format!("Input {}", input.id)));
    }

    if options.is_empty() {
        return Err("No test cases or inputs found for this puzzle".to_string());
    }

    let selected = if options.len() == 1 {
        0
    } else {
        let labels: Vec<_> = options.iter().map(|(_, label)| label.clone()).collect();
        let selection = Select::new("Select option:", labels.clone())
            .prompt()
            .map_err(|_| "Selection cancelled")?;
        labels.iter().position(|s| s == &selection).unwrap()
    };

    let (input_type, description) = &options[selected];
    match input_type {
        InputType::Test(test) => run_test(puzzle, description, &test.input, &test.expected),
        InputType::Input(input) => run_test(puzzle, description, &input.input, &input.expected),
    }

    Ok(())
}

fn run_test(puzzle: &storage::Puzzle, description: &str, input: &str, expected: &str) {
    let start = Instant::now();
    let solution = solve(puzzle.id.year, puzzle.id.day, puzzle.id.part, input);
    let elapsed = start.elapsed();

    match solution {
        Some(result) => {
            println!("=== Solution ===");
            println!("{description}");
            println!(
                "Year: {}, Day: {:02}, Part: {}",
                puzzle.id.year, puzzle.id.day, puzzle.id.part
            );
            println!("Time: {}ms", elapsed.as_millis());
            println!("Result  : {result}");
            println!("Expected: {expected}");

            if result == expected {
                println!("✓ PASS");
            } else {
                println!("✗ FAIL");
            }
        }
        None => {
            println!(
                "Solution not implemented for year {}, day {:02}, part {:02}",
                puzzle.id.year, puzzle.id.day, puzzle.id.part
            );
        }
    }
}

fn select_one<'a, T>(items: &'a [&T], format_fn: fn(&T, usize) -> String) -> Option<&'a T> {
    match items.len() {
        0 => None,
        1 => Some(items[0]),
        _ => {
            let options: Vec<_> = items
                .iter()
                .enumerate()
                .map(|(i, item)| format_fn(item, i))
                .collect();

            let selection = Select::new("Select option:", options.clone())
                .prompt()
                .ok()?;
            let index = options.iter().position(|s| s == &selection)?;
            Some(items[index])
        }
    }
}

fn format_puzzle_option(puzzle: &storage::Puzzle, _: usize) -> String {
    format!(
        "{} Day {:02} Part {} - {}",
        puzzle.id.year, puzzle.id.day, puzzle.id.part, puzzle.metadata.title
    )
}
