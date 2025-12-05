use std::path::Path;
use walkdir::WalkDir;

pub fn generate() -> String {
    println!("cargo:rerun-if-changed=puzzles.enc");
    get_inputs()
        .into_iter()
        .map(|(year, day, part)| {
            format!("#[test]\nfn test_{year}_{day:02}_{part:02}() {{ super::run_test({year}, {day}, {part}); }}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_inputs() -> Vec<(u32, u32, u32)> {
    WalkDir::new(Path::new("puzzles.enc"))
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .filter_map(|e| parse_test_case(&e))
        .collect()
}

fn parse_test_case(entry: &walkdir::DirEntry) -> Option<(u32, u32, u32)> {
    let path = entry.path();

    // Extract day-part from directory name (e.g., "01-1" -> day=1, part=1)
    let dirname = path.file_name()?.to_str()?;
    let mut parts = dirname.split('-');
    let day: u32 = parts.next()?.parse().ok()?;
    let part: u32 = parts.next()?.parse().ok()?;

    // Extract year from parent directory name
    let year: u32 = path.parent()?.file_name()?.to_str()?.parse().ok()?;

    Some((year, day, part))
}
