use std::str::FromStr;

struct Expression {
    operands: Vec<u64>,
    operation: Operation,
}

impl Expression {
    fn compute(&self) -> u64 {
        match self.operation {
            Operation::Addition => self.operands.iter().sum(),
            Operation::Multiplication => self.operands.iter().product(),
        }
    }
}

enum Operation {
    Addition,
    Multiplication,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "+" => Ok(Operation::Addition),
            "*" => Ok(Operation::Multiplication),
            _ => Err(format!("Unknown operation '{value}'")),
        }
    }
}

fn extract_column(input: &[Vec<char>], column: usize) -> Vec<char> {
    input
        .iter()
        .map(|line| line.get(column).copied().unwrap())
        .collect()
}

fn try_parse_operands(col: &[char]) -> Option<u64> {
    let (_, operands) = col.split_last().unwrap();

    operands
        .iter()
        .filter(|&&c| c != ' ')
        .collect::<String>()
        .parse()
        .ok()
}

fn try_parse_operation(col: &[char]) -> Option<Operation> {
    col.last()?.to_string().parse().ok()
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_len = lines.iter().map(Vec::len).max().unwrap();

    let mut expressions = Vec::new();
    let mut operands = Vec::new();
    let mut operation = None;
    for pos in 0..max_len {
        let col = extract_column(&lines, pos);

        let ops = try_parse_operands(&col);
        if ops.is_none() {
            expressions.push(Expression {
                operands,
                operation: operation.unwrap(),
            });
            operands = Vec::new();
            operation = None;
            continue;
        }
        operands.push(ops.unwrap());
        if operation.is_none() {
            operation = try_parse_operation(&col);
        }
    }

    if !operands.is_empty() {
        expressions.push(Expression {
            operands,
            operation: operation.unwrap(),
        });
    }

    expressions.iter().map(Expression::compute).sum()
}
