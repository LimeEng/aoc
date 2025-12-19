use crate::PuzzleId;
use walkdir::WalkDir;

pub fn discover() -> Vec<PuzzleId> {
    WalkDir::new("src/years")
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| parse_solver(&e))
        .collect()
}

fn parse_solver(entry: &walkdir::DirEntry) -> Option<PuzzleId> {
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

    Some(PuzzleId { year, day, part })
}
