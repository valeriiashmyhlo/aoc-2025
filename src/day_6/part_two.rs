use std::char;

use crate::day_6::{Input, Operation};

impl Operation<Vec<char>> {
    fn calc(self) -> i64 {
        let data: Vec<i64> = 
            self.operands
                .iter()
                .rev()
                .map(|chars| 
                    chars.iter().collect::<String>().trim().parse().expect("Failed to parse"))
                .collect();

        match self.operator {
            '+' => data.iter().sum::<i64>(),
            '*' => data.iter().product::<i64>(),
            _ => panic!("Unsupported operator: {:?}", self.operator),
        }
    }
}

impl Input<Vec<char>> {
    fn parse(input: &str) -> Self{
        let mut lines: Vec<Vec<char>> = 
            input
                .lines()
                .map(|line| line.to_string())
                .map(|line| line.chars().collect())
                .collect();
        let last_line = lines.pop().expect("Operators must exist");
        let operators = 
            last_line
                .into_iter()
                .filter(|str| *str != ' ')
                .collect::<Vec<char>>();

        let columns = (0..lines[0].len()).map(|col| {
            lines.iter().map(|line| line[col]).collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

        let mut values: Vec<Vec<Vec<char>>> = Vec::new();
        let mut row_values: Vec<Vec<char>> = Vec::new();

        for col in columns {
            if col.iter().all(|ch| *ch == ' ') {
                values.push(row_values.clone());
                row_values.clear();
                continue;
            }

            row_values.push(col);
        }

        values.push(row_values);
        
        Self { values, operators }
    }

    fn get_operation_values(&self, index: usize) -> Operation<Vec<char>> {
        Operation { operator: self.operators[index], operands: self.values[index].clone() }
    }
}

pub fn solve(input: &str) -> i64 {
    let data = Input::parse(input);
    let mut count = data.values.len();
    let mut sum = 0;

    while count > 0 {
        count -= 1;
        let operation = data.get_operation_values(count);
        let result = operation.calc();
        sum += result;
    }

    sum
}

