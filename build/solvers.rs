use std::fs;
use std::path::Path;

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
    let years_dir = Path::new("src/years");

    fs::read_dir(years_dir)
        .unwrap()
        .filter_map(|entry| {
            let dir = entry.ok()?;
            if !dir.file_type().ok()?.is_dir() {
                return None;
            }

            let year = dir
                .file_name()
                .to_str()?
                .strip_prefix("year_")?
                .parse()
                .ok()?;

            Some((dir.path(), year))
        })
        .flat_map(|(year_path, year)| {
            fs::read_dir(year_path)
                .unwrap()
                .filter_map(move |entry| {
                    let dir = entry.ok()?;
                    if !dir.file_type().ok()?.is_dir() {
                        return None;
                    }

                    let day = dir
                        .file_name()
                        .to_str()?
                        .strip_prefix("day_")?
                        .trim_start_matches('0')
                        .parse()
                        .ok()?;

                    let parts: Vec<_> = fs::read_dir(dir.path())
                        .unwrap()
                        .filter_map(|entry| {
                            let file = entry.ok()?.file_name();
                            let name = file.to_str()?;
                            name.strip_prefix("part_")?
                                .strip_suffix(".rs")?
                                .parse()
                                .ok()
                        })
                        .collect();

                    Some(
                        parts
                            .into_iter()
                            .map(move |part| Solution { year, day, part }),
                    )
                })
                .flatten()
        })
        .collect()
}

fn generate_solvers(solutions: &[Solution]) -> String {
    let mut code = String::from(
        "\
#[must_use]
pub fn solve(year: u32, day: u32, part: u32, input: &str) -> Option<String> {
    match (year, day, part) {\n",
    );

    for solution in solutions {
        code.push_str(&format!(
            "        ({}, {}, {}) => Some(format!(\"{{:?}}\", years::year_{}::day_{:02}::part_{}::solve(input))),\n",
            solution.year, solution.day, solution.part,
            solution.year, solution.day, solution.part
        ));
    }

    code.push_str("        _ => None,\n    }\n}");
    code
}
