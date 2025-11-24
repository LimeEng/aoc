use crate::{
    PuzzleId,
    storage::{ENC_EXTENSION, PUZZLES_DIR, PUZZLES_ENC_DIR, cipher::SaltedKey},
};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use walkdir::WalkDir;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleMetadata {
    pub year: u32,
    pub day: u32,
    pub part: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub id: PuzzleId,
    pub metadata: PuzzleMetadata,
    pub tests: Vec<TestCase>,
}

pub fn get(id: &PuzzleId, password: &str) -> Result<Puzzle, Error> {
    let metadata = read_metadata(id, password)?;
    let tests = read_tests(id, password);

    Ok(Puzzle {
        id: id.clone(),
        metadata,
        tests,
    })
}

pub fn get_all(password: &str) -> Result<Vec<Puzzle>, Error> {
    list()?.into_iter().map(|id| get(&id, password)).collect()
}

pub fn encrypt_all(password: &str) -> Result<(), Error> {
    for entry in WalkDir::new(PUZZLES_DIR)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_plaintext_file)
    {
        encrypt_file(entry.path(), password)?;
    }
    Ok(())
}

pub fn decrypt_all(password: &str) -> Result<(), Error> {
    for entry in WalkDir::new(PUZZLES_ENC_DIR)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_encrypted_file)
    {
        decrypt_file(entry.path(), password)?;
    }
    Ok(())
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

    fs::create_dir_all(output_path.parent().unwrap())?;

    let plaintext = fs::read(plain_path)?;
    let existing_encrypted = fs::read(&output_path).ok();
    let encrypted =
        SaltedKey::reuse_or_new(password, existing_encrypted.as_deref()).encrypt(&plaintext)?;

    fs::write(&output_path, encrypted)?;
    Ok(())
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

fn read_tests(id: &PuzzleId, password: &str) -> Vec<TestCase> {
    let puzzle_dir = Path::new(PUZZLES_ENC_DIR).join(to_puzzle_dir(id));

    let input_suffix = format!(".in.{ENC_EXTENSION}");
    let mut test_data: Vec<(String, TestCase)> = WalkDir::new(&puzzle_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|entry| {
            let filename = entry.file_name().to_str()?;
            if !filename.ends_with(&input_suffix) {
                return None;
            }

            let test_name = filename.strip_suffix(&input_suffix)?;
            let out_path = puzzle_dir.join(format!("{test_name}.out.{ENC_EXTENSION}"));

            // Read and decrypt input
            let input_encrypted = fs::read(entry.path()).ok()?;
            let input = String::from_utf8(
                SaltedKey::extract(password, &input_encrypted)
                    .decrypt(&input_encrypted)
                    .ok()?,
            )
            .ok()?;

            // Read and decrypt output
            let output_encrypted = fs::read(&out_path).ok()?;
            let expected = String::from_utf8(
                SaltedKey::extract(password, &output_encrypted)
                    .decrypt(&output_encrypted)
                    .ok()?,
            )
            .ok()?;

            Some((test_name.to_string(), TestCase { input, expected }))
        })
        .collect();

    // Sort tests by filename for consistent ordering
    test_data.sort_by(|a, b| a.0.cmp(&b.0));

    test_data.into_iter().map(|(_, test)| test).collect()
}

fn read_metadata(id: &PuzzleId, password: &str) -> Result<PuzzleMetadata, Error> {
    let meta_path = Path::new(PUZZLES_ENC_DIR)
        .join(to_puzzle_dir(id))
        .join(format!("meta.toml.{ENC_EXTENSION}"));

    let bytes = fs::read(meta_path)?;
    let toml_bytes = SaltedKey::extract(password, &bytes).decrypt(&bytes)?;
    let metadata: PuzzleMetadata =
        toml::from_str(&String::from_utf8_lossy(&toml_bytes)).map_err(|_| Error::Parse)?;
    Ok(metadata)
}

pub fn list() -> Result<Vec<PuzzleId>, Error> {
    let puzzles_enc = Path::new(PUZZLES_ENC_DIR);
    if !puzzles_enc.exists() {
        return Ok(Vec::new());
    }

    let mut puzzle_ids: Vec<PuzzleId> = WalkDir::new(puzzles_enc)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter_map(|entry| {
            let path = entry.path();
            let dir_name = path.file_name()?.to_str()?;
            let year_name = path.parent()?.file_name()?.to_str()?;

            // Parse year
            let year = year_name.parse::<u32>().ok()?;

            // Parse "DD-P" format
            let mut parts = dir_name.split('-');
            let day = parts.next()?.parse::<u32>().ok()?;
            let part = parts.next()?.parse::<u32>().ok()?;

            Some(PuzzleId::new(year, day, part))
        })
        .collect();

    puzzle_ids.sort_by_key(|id| (id.year, id.day, id.part));
    Ok(puzzle_ids)
}

fn to_puzzle_dir(id: &PuzzleId) -> String {
    format!("{}/{:02}-{}", id.year, id.day, id.part)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Cipher,
    Parse,
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
