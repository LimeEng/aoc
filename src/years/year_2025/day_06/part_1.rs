struct Expression {
    operands: Vec<u64>,
    operation: Operation,
}

impl Expression {
    fn compute(&self) -> u64 {
        let op: fn(u64, u64) -> u64 = match self.operation {
            Operation::Addition => |a, b| a + b,
            Operation::Multiplication => |a, b| a * b,
        };

        self.operands.iter().copied().reduce(op).unwrap()
    }
}

enum Operation {
    Addition,
    Multiplication,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => panic!("Unknown operation '{value}'"),
        }
    }
}

#[must_use]
pub fn solve(input: &str) -> u64 {
    parse(input).iter().map(Expression::compute).sum()
}

fn parse(input: &str) -> Vec<Expression> {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let num_cols = rows.iter().map(Vec::len).max().unwrap();

    (0..num_cols)
        .map(|col_idx| {
            let column: Vec<&str> = rows
                .iter()
                .map(|row| row.get(col_idx).copied().unwrap())
                .collect();

            let (operation, operands) = column.split_last().unwrap();
            let operands = operands.iter().map(|s| s.parse().unwrap()).collect();

            Expression {
                operands,
                operation: Operation::from(*operation),
            }
        })
        .collect()
}
