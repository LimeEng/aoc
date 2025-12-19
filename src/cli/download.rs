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
    let password = env::var("AOC_KEY").map_err(|_| "AOC_KEY environment variable not set")?;
    let session =
        env::var("AOC_SESSION").map_err(|_| "AOC_SESSION environment variable not set")?;

    let year = matches.get_one::<u32>("year").copied().unwrap();
    let day = matches.get_one::<u32>("day").copied().unwrap();
    let part = matches.get_one::<u32>("part").copied().unwrap();

    let id = PuzzleId::new(year, day, part);

    let needs_prompt = check_needs_prompt(&id, &password);
    let needs_input = check_needs_input(&id, &password);

    if !needs_prompt && !needs_input {
        println!("Puzzle {year} day {day:02} part {part} already downloaded");
        return Ok(());
    }

    let api = AdventOfCode::new(&session).map_err(|_| "Failed to initialize API client")?;

    let downloaded_prompt = if needs_prompt {
        download_prompt(&api, &id, &password)?;
        true
    } else {
        false
    };

    let downloaded_input = if needs_input {
        download_input(&api, &id, &password)?;
        true
    } else {
        false
    };

    match (downloaded_prompt, downloaded_input) {
        (true, true) => println!("Downloaded puzzle and input for {year} day {day:02} part {part}"),
        (true, false) => println!("Downloaded puzzle for {year} day {day:02} part {part}"),
        (false, true) => println!("Downloaded input for {year} day {day:02} part {part}"),
        (false, false) => unreachable!(),
    }
    Ok(())
}

fn check_needs_prompt(id: &PuzzleId, password: &str) -> bool {
    storage::get_description(id, password)
        .map(|desc| desc.description.is_empty())
        .unwrap_or(true)
}

fn check_needs_input(id: &PuzzleId, password: &str) -> bool {
    storage::read_input(id, password).is_empty()
}

fn download_prompt(api: &AdventOfCode, id: &PuzzleId, password: &str) -> Result<(), String> {
    let prompt = api.get_puzzle(id).map_err(|e| {
        format!(
            "Failed to download puzzle {} day {:02} part {}: {e:?}",
            id.year, id.day, id.part
        )
    })?;

    storage::save_description(
        id,
        Some(&prompt.description),
        Some(&prompt.styles),
        password,
    )
    .map_err(|_| "Failed to save puzzle description")?;

    Ok(())
}

fn download_input(api: &AdventOfCode, id: &PuzzleId, password: &str) -> Result<(), String> {
    let input = api.get_input(id).map_err(|e| {
        format!(
            "Failed to download input for {} day {:02} part {}: {e:?}",
            id.year, id.day, id.part
        )
    })?;

    storage::save_input(id, 1, &input, None, password)
        .map_err(|_| "Failed to save puzzle input")?;

    Ok(())
}
