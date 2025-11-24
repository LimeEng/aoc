use walkdir::WalkDir;

#[derive(Debug)]
struct Solution {
    year: i32,
    day: i32,
    part: i32,
}

pub fn generate() -> String {
    println!("cargo:rerun-if-changed=src/years");
    let solutions = discover_solutions();
    generate_solvers(&solutions)
}

fn discover_solutions() -> Vec<Solution> {
    WalkDir::new("src/years")
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| parse_solution(&e))
        .collect()
}

fn parse_solution(entry: &walkdir::DirEntry) -> Option<Solution> {
    let path = entry.path();

    // Extract part from filename (e.g., "part_1.rs" -> 1)
    let part = path
        .file_stem()?
        .to_str()?
        .strip_prefix("part_")?
        .parse()
        .ok()?;

    // Extract day from parent directory name (e.g., "day_01" -> 1)
    let day = path
        .parent()?
        .file_name()?
        .to_str()?
        .strip_prefix("day_")?
        .trim_start_matches('0')
        .parse()
        .ok()?;

    // Extract year from grandparent directory name (e.g., "year_2024" -> 2024)
    let year = path
        .parent()?
        .parent()?
        .file_name()?
        .to_str()?
        .strip_prefix("year_")?
        .parse()
        .ok()?;

    Some(Solution { year, day, part })
}

fn generate_solvers(solutions: &[Solution]) -> String {
    use std::fmt::Write;

    let mut code = String::from(
        "\
#[must_use]
pub fn solve(year: u32, day: u32, part: u32, input: &str) -> Option<String> {
    match (year, day, part) {\n",
    );

    for solution in solutions {
        writeln!(
            code,
            "        ({}, {}, {}) => Some(format!(\"{{}}\", years::year_{}::day_{:02}::part_{}::solve(input))),",
            solution.year, solution.day, solution.part,
            solution.year, solution.day, solution.part
        ).unwrap();
    }

    code.push_str("        _ => None,\n    }\n}");
    code
}
