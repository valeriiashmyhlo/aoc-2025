use crate::day_6::{Input, Operation};

impl Operation<i64> {
    fn calc(self) -> i64 {
        let operands = self.operands.into_iter();

        match self.operator {
            '+' => operands.sum(),
            '*' => operands.product(),
            _ => panic!("Unsupported operator: {:?}", self.operator),
        }
    }
}

impl Input<i64> {
    fn parse(input: &str) -> Self {
        let mut lines: Vec<Vec<&str>> = 
        input
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();
        let last_line = lines.pop().expect("Operators must exist");
        let operators = 
            last_line
                .into_iter()
                .map(|str| str.chars().next().expect("Operator is required"))
                .collect::<Vec<char>>();
        let values: Vec<Vec<i64>> = 
            lines
                .into_iter()
                .map(|line| 
                    line
                        .iter()
                        .map(|l| l.parse().expect("Failed to parse"))
                        .collect())
                .collect();

        Self { values, operators }
    }

    fn get_operation_values(&self, index: usize) -> Operation<i64> {
        let mut operands: Vec<i64> = Vec::new();

        for row in self.values.iter() {
            operands.push(row[index]);
        }

        Operation { operator: self.operators[index], operands }
    }
}

pub fn solve(input: &str) -> i64 {
    let data = Input::parse(input);
    let mut count: usize = 0;
    let mut sum: i64 = 0;

    while count < data.values[0].len() {
        let operation = data.get_operation_values(count);
        let result = operation.calc();
        sum += result;
        count += 1;
    }

    sum
}
