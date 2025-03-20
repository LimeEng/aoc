use aoc::input_storage::{self, EntryId, TestEntry};
use clap::{Arg, ArgMatches, Command, crate_name, crate_version, value_parser};
use std::env;
use std::path::Path;
use std::{fs, time::Instant};

// aoc solve 2024 05 02 input.txt
// aoc decrypt-inputs
// aoc test add 2024 05 02 input.txt output.txt

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .long_version(crate_version!())
        .propagate_version(true)
        .arg_required_else_help(true)
        .about("Advent of Code runner")
        .subcommand_required(true)
        .subcommand(
            Command::new("solve")
                .arg(
                    Arg::new("year")
                        .required(true)
                        .value_parser(value_parser!(u32)),
                )
                .arg(
                    Arg::new("day")
                        .required(true)
                        .value_parser(value_parser!(u32)),
                )
                .arg(
                    Arg::new("part")
                        .required(true)
                        .value_parser(value_parser!(u32)),
                )
                .arg(Arg::new("input").required(true)),
        )
        .subcommand(
            Command::new("decrypt")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(Command::new("inputs")),
        )
        .subcommand(
            Command::new("test")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(
                    Command::new("add")
                        .arg(
                            Arg::new("year")
                                .required(true)
                                .value_parser(value_parser!(u32)),
                        )
                        .arg(
                            Arg::new("day")
                                .required(true)
                                .value_parser(value_parser!(u32)),
                        )
                        .arg(
                            Arg::new("part")
                                .required(true)
                                .value_parser(value_parser!(u32)),
                        )
                        .arg(Arg::new("input").required(true))
                        .arg(Arg::new("expected").required(true)),
                ),
        )
        .get_matches();

    unsafe { env::set_var("AOC_KEY", "b;6&@qNdm;h1BcF3gDH,") };

    let unreachable_message =
        "Exhausted list of subcommands and subcommand_required prevents `None`";

    match matches.subcommand() {
        Some(("solve", matches)) => solve(matches),
        Some(("decrypt", matches)) => match matches.subcommand() {
            Some(("inputs", _)) => decrypt(),
            _ => unreachable!("{unreachable_message}"),
        },
        Some(("test", matches)) => match matches.subcommand() {
            Some(("add", matches)) => test_add(matches),
            _ => unreachable!("{unreachable_message}"),
        },
        _ => unreachable!("{unreachable_message}"),
    }
}

fn solve(matches: &ArgMatches) {
    // This should be safe since they are all .required(true)
    let year: u32 = *matches.get_one::<u32>("year").expect("'year' is required");
    let day: u32 = *matches.get_one::<u32>("day").expect("'day' is required");
    let part: u32 = *matches.get_one::<u32>("part").expect("'part' is required");
    let input = matches
        .get_one::<String>("input")
        .expect("'input' is required");

    let contents = fs::read_to_string(input);
    if let Ok(contents) = contents {
        let start = Instant::now();
        let solution = aoc::solve(year, day, part, &contents);
        let elapsed = start.elapsed();

        match solution {
            Some(result) => {
                println!("Solution for {year}, day {day:02}, part {part:02}");
                println!("Time: {}ms", elapsed.as_millis());
                println!("{result}");
            }
            None => {
                println!("Solution not implemented for year {year}, day {day:02}, part {part:02}",);
            }
        }
    } else {
        println!("Failed to read {input}",);
    }
}

fn decrypt() {
    let password = env::var("AOC_KEY").unwrap();
    let output_dir = Path::new("inputs_decrypted");

    let entry_ids = input_storage::list().unwrap();
    for id in &entry_ids {
        let mut entries = input_storage::read(id, &password).unwrap();
        // Just for cosmetic reasons
        entries.sort_by_key(|tc| tc.input.len() + tc.expected.len());
        let path = output_dir.join(id.to_path());
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let json = serde_json::to_string(&entries).unwrap();
        fs::write(path, json).unwrap();
    }
}

fn test_add(matches: &ArgMatches) {
    // This should be safe since they are all .required(true)
    let year: u32 = *matches.get_one::<u32>("year").expect("'year' is required");
    let day: u32 = *matches.get_one::<u32>("day").expect("'day' is required");
    let part: u32 = *matches.get_one::<u32>("part").expect("'part' is required");
    let input = matches
        .get_one::<String>("input")
        .expect("'input' is required");
    let expected = matches
        .get_one::<String>("expected")
        .expect("'expected' is required");

    let Ok(input_contents) = fs::read_to_string(input) else {
        println!("Failed to read {input}");
        return;
    };

    let Ok(expected_contents) = fs::read_to_string(expected) else {
        println!("Failed to read {expected}");
        return;
    };

    let test_case = TestEntry {
        input: input_contents,
        expected: expected_contents,
    };
    let password = env::var("AOC_KEY").unwrap();
    let test_id = EntryId::new(year, day, part);

    match input_storage::write(&test_id, &password, test_case) {
        Ok(()) => println!("Test case successfully added!"),
        Err(_) => println!("Failed to add test case"),
    }
}
