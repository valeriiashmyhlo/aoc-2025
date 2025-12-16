pub mod part_one;
pub mod part_two;
pub use part_one::solve as part_one;
pub use part_two::solve as part_two;

pub const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.to_string()).map(|line| line.chars().collect()).collect()
}
