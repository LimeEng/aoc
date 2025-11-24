use crate::storage;
use clap::{ArgMatches, Command};
use std::env;

#[must_use]
pub fn command() -> Command {
    Command::new("decrypt")
}

pub fn execute(_matches: &ArgMatches) {
    let password = env::var("AOC_KEY").unwrap();
    storage::decrypt_all(&password).unwrap();
    println!("All files decrypted successfully!");
}
