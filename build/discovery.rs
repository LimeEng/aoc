use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct PuzzleInfo {
    pub year: u32,
    pub day: u32,
    pub part: u32,
}

/// Discover solvers from src/years directory structure
pub fn discover_solvers() -> Vec<PuzzleInfo> {
    WalkDir::new("src/years")
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| parse_solver(&e))
        .collect()
}

fn parse_solver(entry: &walkdir::DirEntry) -> Option<PuzzleInfo> {
    let path = entry.path();

    let part = path
        .file_stem()?
        .to_str()?
        .strip_prefix("part_")?
        .parse()
        .ok()?;

    let day = path
        .parent()?
        .file_name()?
        .to_str()?
        .strip_prefix("day_")?
        .trim_start_matches('0')
        .parse()
        .ok()?;

    let year = path
        .parent()?
        .parent()?
        .file_name()?
        .to_str()?
        .strip_prefix("year_")?
        .parse()
        .ok()?;

    Some(PuzzleInfo { year, day, part })
}

pub fn discover_tests() -> Vec<PuzzleInfo> {
    WalkDir::new(Path::new("puzzles.enc"))
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter(|e| {
            // Only include directories named "tests"
            e.file_name().to_str() == Some("tests")
        })
        .filter_map(|e| parse_test(&e))
        .collect()
}

fn parse_test(entry: &walkdir::DirEntry) -> Option<PuzzleInfo> {
    let path = entry.path();

    let part = path
        .parent()?
        .file_name()?
        .to_str()?
        .strip_prefix("part_")?
        .parse()
        .ok()?;

    let day = path
        .parent()?
        .parent()?
        .file_name()?
        .to_str()?
        .parse()
        .ok()?;

    let year = path
        .parent()?
        .parent()?
        .parent()?
        .file_name()?
        .to_str()?
        .parse()
        .ok()?;

    Some(PuzzleInfo { year, day, part })
}
