use crate::cipher::SaltedKey;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestEntry {
    pub input: String,
    pub expected: String,
}

#[derive(Debug)]
pub struct EntryId {
    year: u32,
    day: u32,
    part: u32,
}

impl EntryId {
    #[must_use]
    pub fn new(year: u32, day: u32, part: u32) -> Self {
        Self { year, day, part }
    }

    #[must_use]
    pub fn to_path(&self) -> String {
        format!("{}/{:02}/{:02}.json", self.year, self.day, self.part)
    }
}

pub fn read(id: &EntryId, password: &str) -> io::Result<Vec<TestEntry>> {
    let target_path = Path::new("inputs").join(id.to_path());

    let bytes = fs::read(target_path)?;
    let json = SaltedKey::extract(password, &bytes).decrypt(&bytes);
    let test_cases = serde_json::from_slice(&json).unwrap();
    Ok(test_cases)
}

pub fn write(id: &EntryId, password: &str, test_case: TestEntry) -> io::Result<()> {
    let target_path = Path::new("inputs").join(id.to_path());

    let mut existing = read(id, password).unwrap_or_else(|_| Vec::new());
    existing.push(test_case);

    let json = serde_json::to_string(&existing).unwrap();
    let encrypted = SaltedKey::new(password).encrypt(json.as_bytes());
    fs::create_dir_all(target_path.parent().unwrap())?;
    fs::write(&target_path, encrypted)
}

pub fn list() -> io::Result<Vec<EntryId>> {
    let re = Regex::new(r"inputs/(?P<year>\d{4})/(?P<day>\d{2})/(?P<part>\d{2})").unwrap();

    let t = glob::glob("inputs/*/*/*.json")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|path| {
            // TODO: This line in particular is pretty hacky.
            // The enclosing function isn't very nice either
            let path = path.to_string_lossy().into_owned().replace('\\', "/");
            re.captures(&path).map(|captures| {
                EntryId::new(
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                )
            })
        })
        .collect();

    Ok(t)
}
