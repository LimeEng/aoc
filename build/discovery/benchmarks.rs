use crate::PuzzleId;
use walkdir::WalkDir;

pub fn discover() -> Vec<PuzzleId> {
    WalkDir::new("puzzles.enc")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter(|e| e.file_name().to_str() == Some("inputs"))
        .filter_map(|e| parse_puzzle(&e))
        .collect()
}

fn parse_puzzle(entry: &walkdir::DirEntry) -> Option<PuzzleId> {
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

    Some(PuzzleId { year, day, part })
}
