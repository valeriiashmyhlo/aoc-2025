use super::{merge_ranges, parse_input};

pub fn solve(input: &str) -> u64 {
    let pantry = parse_input(input);
    let merged = merge_ranges(pantry.ranges);

    merged
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}
