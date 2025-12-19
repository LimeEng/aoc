use clap::{Command, crate_name, crate_version};

pub mod decrypt;
pub mod download;
pub mod encrypt;
pub mod solve;

pub fn run() {
    let mut app = build_cli();
    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("decrypt", matches)) => decrypt::execute(matches),
        Some(("download", matches)) => download::execute(matches),
        Some(("encrypt", matches)) => encrypt::execute(matches),
        Some(("solve", matches)) => solve::execute(matches),
        None => println!("No subcommand was used"),
        _ => app.print_help().unwrap(),
    }
}

fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Advent of Code runner")
        .subcommand_required(true)
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(decrypt::command())
        .subcommand(download::command())
        .subcommand(encrypt::command())
        .subcommand(solve::command())
}
