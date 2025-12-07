use crate::{PuzzleId, api::AdventOfCode, storage};
use clap::{Arg, ArgMatches, Command, value_parser};
use std::env;

#[must_use]
pub fn command() -> Command {
    Command::new("download")
        .about("Download puzzle description and styles")
        .arg(
            Arg::new("year")
                .value_parser(value_parser!(u32))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("day")
                .value_parser(value_parser!(u32))
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("part")
                .value_parser(value_parser!(u32))
                .required(true)
                .index(3),
        )
}

pub fn execute(matches: &ArgMatches) {
    if let Err(e) = try_execute(matches) {
        println!("Error: {e}");
    }
}

fn try_execute(matches: &ArgMatches) -> Result<(), String> {
    let session =
        env::var("AOC_SESSION").map_err(|_| "AOC_SESSION environment variable not set")?;

    let year = matches.get_one::<u32>("year").copied().unwrap();
    let day = matches.get_one::<u32>("day").copied().unwrap();
    let part = matches.get_one::<u32>("part").copied().unwrap();

    // Validate puzzle ID
    let _id = PuzzleId::new(year, day, part);

    let api = AdventOfCode::new(&session).map_err(|_| "Failed to initialize API client")?;

    let prompt = if part == 1 {
        api.part_1_prompt(year, day)
    } else {
        api.part_2_prompt(year, day)
    }
    .map_err(|_| format!("Failed to download puzzle {year} day {day:02} part {part}"))?;

    storage::save_description(
        year,
        day,
        part,
        Some(&prompt.description),
        Some(&prompt.styles),
    )
    .map_err(|_| "Failed to save puzzle description")?;

    println!("Downloaded puzzle {year} day {day:02} part {part}");
    Ok(())
}
