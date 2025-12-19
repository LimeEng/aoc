use super::TestCase;
use crate::{
    PuzzleId,
    storage::{ENC_EXTENSION, PUZZLES_ENC_DIR, components::read_encrypted},
};
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

enum FileType {
    Input(u32),
    Output(u32),
}

fn parse_test_filename(filename: &str) -> Option<FileType> {
    let rest = filename.strip_prefix("test_")?;

    if let Some(id) = rest.strip_suffix(&format!(".in.{ENC_EXTENSION}")) {
        let id = id.parse::<u32>().ok()?;
        Some(FileType::Input(id))
    } else if let Some(id) = rest.strip_suffix(&format!(".out.{ENC_EXTENSION}")) {
        let id = id.parse::<u32>().ok()?;
        Some(FileType::Output(id))
    } else {
        None
    }
}

fn read_file_content(path: &Path, password: &str) -> Option<String> {
    read_encrypted(path, password)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
}

fn collect_test_pairs(
    test_dir: &Path,
    password: &str,
) -> HashMap<u32, (Option<String>, Option<String>)> {
    let mut tests: HashMap<u32, (Option<String>, Option<String>)> = HashMap::new();

    for entry in WalkDir::new(test_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let Some(filename) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        match parse_test_filename(filename) {
            Some(FileType::Input(id)) => {
                let content = read_file_content(path, password);
                tests.entry(id).or_default().0 = content;
            }
            Some(FileType::Output(id)) => {
                let content = read_file_content(path, password);
                tests.entry(id).or_default().1 = content;
            }
            None => {}
        }
    }

    tests
}

fn build_test_cases(tests: HashMap<u32, (Option<String>, Option<String>)>) -> Vec<TestCase> {
    let mut result: Vec<_> = tests
        .into_iter()
        .filter_map(|(id, (input, expected))| {
            Some(TestCase {
                id,
                input: input?,
                expected: expected?,
            })
        })
        .collect();

    result.sort_by_key(|t| t.id);
    result
}

pub fn read_tests(id: &PuzzleId, password: &str) -> Vec<TestCase> {
    let test_dir = Path::new(PUZZLES_ENC_DIR)
        .join(format!("{}/{:02}/part_{}/tests", id.year, id.day, id.part));

    let tests = collect_test_pairs(&test_dir, password);
    build_test_cases(tests)
}
