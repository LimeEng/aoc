use std::collections::HashMap;

struct Machine {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let machines = parse(input);

    machines.iter().map(solve_machine).sum()
}

fn solve_machine(machine: &Machine) -> u64 {
    let num_counters = machine.joltage.len();

    // Convert buttons to coefficient form (1 if button affects counter i, 0 otherwise)
    let coeffs: Vec<Vec<u64>> = machine
        .buttons
        .iter()
        .map(|button| {
            (0..num_counters)
                .map(|i| u64::from(button.contains(&i)))
                .collect()
        })
        .collect();

    let pattern_costs = compute_patterns(&coeffs);

    let mut memo = HashMap::new();
    solve_recursive(&machine.joltage, &pattern_costs, &mut memo)
}

fn compute_patterns(coeffs: &[Vec<u64>]) -> HashMap<Vec<u64>, HashMap<Vec<u64>, u64>> {
    let num_buttons = coeffs.len();
    let num_counters = coeffs[0].len();

    let mut patterns: HashMap<Vec<u64>, HashMap<Vec<u64>, u64>> = HashMap::new();

    for mask in 0..(1u64 << num_buttons) {
        let num_pressed = u64::from(mask.count_ones());

        let mut pattern = vec![0u64; num_counters];
        for (button_idx, button_coeffs) in coeffs.iter().enumerate() {
            if (mask >> button_idx) & 1 == 1 {
                for (counter_idx, &coeff) in button_coeffs.iter().enumerate() {
                    pattern[counter_idx] += coeff;
                }
            }
        }

        let parity_pattern: Vec<u64> = pattern.iter().map(|&x| x % 2).collect();
        let parity_entry = patterns.entry(parity_pattern).or_default();

        parity_entry
            .entry(pattern)
            .and_modify(|cost| *cost = (*cost).min(num_pressed))
            .or_insert(num_pressed);
    }

    patterns
}

fn solve_recursive(
    goal: &[u64],
    pattern_costs: &HashMap<Vec<u64>, HashMap<Vec<u64>, u64>>,
    memo: &mut HashMap<Vec<u64>, u64>,
) -> u64 {
    if goal.iter().all(|&x| x == 0) {
        return 0;
    }

    if let Some(&cached) = memo.get(goal) {
        return cached;
    }

    let mut answer = u64::MAX;

    let parity: Vec<u64> = goal.iter().map(|&x| x % 2).collect();

    if let Some(patterns) = pattern_costs.get(&parity) {
        for (pattern, &pattern_cost) in patterns {
            if pattern.iter().zip(goal.iter()).all(|(&p, &g)| p <= g) {
                let new_goal: Vec<u64> = goal
                    .iter()
                    .zip(pattern.iter())
                    .map(|(&g, &p)| (g - p) / 2)
                    .collect();

                let sub_cost = solve_recursive(&new_goal, pattern_costs, memo);
                if sub_cost != u64::MAX {
                    answer = answer.min(pattern_cost + 2 * sub_cost);
                }
            }
        }
    }

    memo.insert(goal.to_vec(), answer);
    answer
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (_lights, line) = line.split_once(' ').unwrap();

            let (buttons_str, joltage_str) = line.rsplit_once(' ').unwrap();
            let buttons = buttons_str
                .split_whitespace()
                .map(|button| {
                    button
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltage = joltage_str
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            Machine { buttons, joltage }
        })
        .collect()
}
