pub mod part_one;
pub mod part_two;
pub use part_one::solve as part_one;
pub use part_two::solve as part_two;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Operation<T> {
    operator: char,
    operands: Vec<T>,
}

#[derive(Debug)]
struct Input<T> {
    values: Vec<Vec<T>>,
    operators: Vec<char>,
}
