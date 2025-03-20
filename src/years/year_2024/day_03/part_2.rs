use regex::Regex;

#[must_use]
pub fn solve(input: &str) -> i32 {
    let mul_pattern = r"mul\((?P<term_1>\d{1,3}),(?P<term_2>\d{1,3})\)";
    let control_pattern = r"(?P<control>do\(\)|don't\(\))";
    let pattern = format!("{mul_pattern}|{control_pattern}");

    let regex = Regex::new(&pattern).unwrap();

    let mut operations: Vec<(i32, i32)> = vec![];
    let mut enabled = true;
    for caps in regex.captures_iter(input) {
        if let Some(term_1) = caps.name("term_1") {
            let term_2 = caps.name("term_2").unwrap();
            if enabled {
                operations.push((
                    term_1.as_str().parse().unwrap(),
                    term_2.as_str().parse().unwrap(),
                ));
            }
        } else if let Some(control) = caps.name("control") {
            match control.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                pattern => panic!("Unknown control pattern: [{pattern}]"),
            }
        }
    }

    operations.iter().map(|(t1, t2)| t1 * t2).sum()
}
