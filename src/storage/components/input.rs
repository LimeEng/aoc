use super::{Error, PuzzleInput, read_encrypted, write_encrypted};
use crate::{
    PuzzleId,
    storage::{ENC_EXTENSION, PUZZLES_ENC_DIR},
};
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

enum FileType {
    Input(u32),
    Output(u32),
}

fn parse_puzzle_filename(filename: &str) -> Option<FileType> {
    let rest = filename.strip_prefix("puzzle_")?;

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

fn collect_input_pairs(
    input_dir: &Path,
    password: &str,
) -> HashMap<u32, (Option<String>, Option<String>)> {
    let mut inputs: HashMap<u32, (Option<String>, Option<String>)> = HashMap::new();

    for entry in WalkDir::new(input_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let Some(filename) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        match parse_puzzle_filename(filename) {
            Some(FileType::Input(id)) => {
                let content = read_file_content(path, password);
                inputs.entry(id).or_default().0 = content;
            }
            Some(FileType::Output(id)) => {
                let content = read_file_content(path, password);
                inputs.entry(id).or_default().1 = content;
            }
            None => {}
        }
    }

    inputs
}

fn build_puzzle_inputs(inputs: HashMap<u32, (Option<String>, Option<String>)>) -> Vec<PuzzleInput> {
    let mut result: Vec<_> = inputs
        .into_iter()
        .filter_map(|(id, (input, expected))| {
            Some(PuzzleInput {
                id,
                input: input?,
                expected: expected?,
            })
        })
        .collect();

    result.sort_by_key(|p| p.id);
    result
}

#[must_use]
pub fn read_input(id: &PuzzleId, password: &str) -> Vec<PuzzleInput> {
    let input_dir = Path::new(PUZZLES_ENC_DIR)
        .join(format!("{}/{:02}/part_{}/inputs", id.year, id.day, id.part));

    let inputs = collect_input_pairs(&input_dir, password);
    build_puzzle_inputs(inputs)
}

pub fn save_input(
    id: &PuzzleId,
    input_id: u32,
    input: &str,
    expected: Option<&str>,
    password: &str,
) -> Result<(), Error> {
    let input_dir = Path::new(PUZZLES_ENC_DIR)
        .join(format!("{}/{:02}/part_{}/inputs", id.year, id.day, id.part));

    let input_path = input_dir.join(format!("puzzle_{input_id:02}.in.{ENC_EXTENSION}"));
    write_encrypted(&input_path, input.as_bytes(), password)?;

    if let Some(output) = expected {
        let output_path = input_dir.join(format!("puzzle_{input_id:02}.out.{ENC_EXTENSION}"));
        write_encrypted(&output_path, output.as_bytes(), password)?;
    }

    Ok(())
}
