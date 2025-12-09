struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u64>>,
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let machines = parse(input);

    machines.iter().map(solve_machine).sum()
}

fn solve_machine(machine: &Machine) -> u64 {
    let mut min_presses = u64::MAX;

    for mask in 0..(1 << machine.buttons.len()) {
        let mut state = vec![false; machine.lights.len()];
        let mut presses = 0;

        for button_index in 0..machine.buttons.len() {
            if (mask >> button_index) & 1 == 1 {
                presses += 1;
                for &light_index in &machine.buttons[button_index] {
                    let index = usize::try_from(light_index).unwrap();
                    state[index] = !state[index];
                }
            }
        }

        if state == machine.lights {
            min_presses = min_presses.min(presses);
        }
    }

    min_presses
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (lights, line) = line.split_once(' ').unwrap();
            let lights = lights
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .map(|c| c == '#')
                .collect();

            let (buttons, _joltage) = line.rsplit_once(' ').unwrap();
            let buttons = buttons
                .split_whitespace()
                .map(|button| {
                    button
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();

            Machine { lights, buttons }
        })
        .collect()
}
