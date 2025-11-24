use crate::storage;
use clap::{ArgMatches, Command};
use std::env;

#[must_use]
pub fn command() -> Command {
    Command::new("encrypt")
}

pub fn execute(_matches: &ArgMatches) {
    let password = env::var("AOC_KEY").unwrap();
    storage::encrypt_all(&password).unwrap();
    println!("All files encrypted successfully!");
}
