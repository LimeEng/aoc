use crate::{
    PuzzleId,
    storage::{ENC_EXTENSION, PUZZLES_DIR, PUZZLES_ENC_DIR, cipher::SaltedKey},
};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use walkdir::WalkDir;

mod input;
mod metadata;
mod puzzle;
mod tests;

pub use input::*;
pub use metadata::*;
pub use puzzle::*;
pub use tests::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub id: u32,
    pub input: String,
    pub expected: String,
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub id: u32,
    pub input: String,
    pub expected: String,
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub id: PuzzleId,
    pub metadata: PuzzleMetadata,
    pub tests: Vec<TestCase>,
    pub inputs: Vec<PuzzleInput>,
    pub description: Option<Description>,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Cipher,
    Parse,
    Utf8(std::string::FromUtf8Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<aes_gcm_siv::Error> for Error {
    fn from(_error: aes_gcm_siv::Error) -> Self {
        Error::Cipher
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::Utf8(error)
    }
}

pub fn read_encrypted(path: &Path, password: &str) -> Result<Vec<u8>, Error> {
    let bytes = fs::read(path)?;
    let bytes = SaltedKey::extract(password, &bytes).decrypt(&bytes)?;
    Ok(bytes)
}

pub fn write_encrypted(path: &Path, contents: &[u8], password: &str) -> Result<(), Error> {
    let existing_encrypted = fs::read(path).ok();
    let encrypted =
        SaltedKey::reuse_or_new(password, existing_encrypted.as_deref()).encrypt(contents)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, encrypted)?;
    Ok(())
}

pub fn get(id: &PuzzleId, password: &str) -> Result<Puzzle, Error> {
    let metadata = read_metadata(id, password)?;
    let tests = read_tests(id, password);
    let inputs = read_input(id, password);
    let description = get_description(id, password).ok();

    Ok(Puzzle {
        id: id.clone(),
        metadata,
        tests,
        inputs,
        description,
    })
}

pub fn get_all(password: &str) -> Result<Vec<Puzzle>, Error> {
    list().into_iter().map(|id| get(&id, password)).collect()
}

pub fn encrypt_all(password: &str) -> Result<(), Error> {
    WalkDir::new(PUZZLES_DIR)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_plaintext_file)
        .try_for_each(|entry| encrypt_file(entry.path(), password))
}

pub fn decrypt_all(password: &str) -> Result<(), Error> {
    WalkDir::new(PUZZLES_ENC_DIR)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_encrypted_file)
        .try_for_each(|entry| decrypt_file(entry.path(), password))
}

fn is_plaintext_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .path()
            .extension()
            .is_none_or(|ext| ext != ENC_EXTENSION)
}

fn is_encrypted_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .path()
            .extension()
            .is_some_and(|ext| ext == ENC_EXTENSION)
}

fn encrypt_file(plain_path: &Path, password: &str) -> Result<(), Error> {
    let output_path = Path::new(PUZZLES_ENC_DIR)
        .join(plain_path.strip_prefix(PUZZLES_DIR).unwrap())
        .with_extension(
            plain_path
                .extension()
                .map_or(ENC_EXTENSION.to_string(), |ext| {
                    format!("{}.{ENC_EXTENSION}", ext.to_str().unwrap())
                }),
        );

    let plaintext = fs::read(plain_path)?;
    write_encrypted(&output_path, &plaintext, password)
}

fn decrypt_file(enc_path: &Path, password: &str) -> Result<(), Error> {
    let output_path = Path::new(PUZZLES_DIR)
        .join(enc_path.strip_prefix(PUZZLES_ENC_DIR).unwrap())
        .with_extension("");

    fs::create_dir_all(output_path.parent().unwrap())?;

    let encrypted = fs::read(enc_path)?;
    let decrypted = SaltedKey::extract(password, &encrypted).decrypt(&encrypted)?;

    fs::write(&output_path, decrypted)?;
    Ok(())
}

pub fn list() -> Vec<PuzzleId> {
    let puzzles_enc = Path::new(PUZZLES_ENC_DIR);
    if !puzzles_enc.exists() {
        return Vec::new();
    }

    let mut puzzle_ids: Vec<PuzzleId> = WalkDir::new(puzzles_enc)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter(|e| {
            e.file_name()
                .to_str()
                .is_some_and(|name| name.starts_with("part_"))
        })
        .filter_map(|entry| {
            let path = entry.path();
            let part_dir = path.file_name()?.to_str()?;
            let day_dir = path.parent()?.file_name()?.to_str()?;
            let year_dir = path.parent()?.parent()?.file_name()?.to_str()?;

            let year = year_dir.parse::<u32>().ok()?;
            let day = day_dir.parse::<u32>().ok()?;
            let part = part_dir.strip_prefix("part_")?.parse::<u32>().ok()?;

            Some(PuzzleId::new(year, day, part))
        })
        .collect();

    puzzle_ids.sort_by_key(|id| (id.year, id.day, id.part));
    puzzle_ids
}
