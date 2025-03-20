use std::fs;

pub fn generate() -> String {
    println!("cargo:rerun-if-changed=inputs");
    let inputs = get_inputs();
    let t:Vec<_> = inputs.iter().map(|(year, day, part)| {
        format!("#[test]\nfn test_{year}_{day:02}_{part:02}() {{ super::run_test({year}, {day}, {part}); }}")
    }).collect();
    t.join("\n")
}

fn get_inputs() -> Vec<(u32, u32, u32)> {
    let input_dir = "inputs";

    let mut test_cases = Vec::new();

    // TODO: ChatGPT code is something else.
    // It works though!
    // I think
    if let Ok(entries) = fs::read_dir(input_dir) {
        for year_entry in entries.flatten() {
            if let Ok(year) = year_entry.file_name().into_string().unwrap().parse::<u32>() {
                let year_path = year_entry.path();
                if year_path.is_dir() {
                    if let Ok(days) = fs::read_dir(&year_path) {
                        for day_entry in days.flatten() {
                            if let Ok(day) =
                                day_entry.file_name().into_string().unwrap().parse::<u32>()
                            {
                                let day_path = day_entry.path();
                                if day_path.is_dir() {
                                    if let Ok(parts) = fs::read_dir(&day_path) {
                                        for part_entry in parts.flatten() {
                                            let part_file = part_entry.file_name();
                                            if let Some(part_str) = part_file.to_str() {
                                                if let Some(part) = part_str.strip_suffix(".json") {
                                                    if let Ok(part) = part.parse::<u32>() {
                                                        test_cases.push((year, day, part));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    test_cases
}
