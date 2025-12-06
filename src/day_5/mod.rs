use std::ops::Range;

pub mod part_one;
pub mod part_two;
pub use part_one::solve as part_one;
pub use part_two::solve as part_two;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ingredient {
    pub value: u64,
}

#[derive(Debug)]
pub struct Pantry {
    pub ranges: Vec<Range<u64>>,
    pub ingredients: Vec<Ingredient>,
}

pub fn parse_input(input: &str) -> Pantry {
    input.lines().fold(
        Pantry { ranges: Vec::new(), ingredients: Vec::new() },
        |mut acc, line| {
            let parts: Vec<&str> = line.split('-').collect();

            match parts.as_slice() {
                [start, end] => {
                    if let (Ok(start), Ok(end)) = (start.parse(), end.parse()) {
                        acc.ranges.push(Range { start, end });
                    } else {
                        panic!("Invalid range line: {}", line);
                    }
                }
                [value] => {
                    if let Ok(value) = value.parse() {
                        acc.ingredients.push(Ingredient { value });
                    }
                }
                _ => panic!("Invalid input line: {}", line),
            }

            acc
        },
    )
}

pub fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged: Vec<Range<u64>> = Vec::new();

    for range in ranges.into_iter() {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_ranges() {
        let ranges = vec![
            Range { start: 1, end: 5 },
            Range { start: 3, end: 7 },
            Range { start: 8, end: 10 },
            Range { start: 9, end: 12 },
        ];

        let merged = merge_ranges(ranges);

        assert_eq!(
            merged,
            vec![
                Range { start: 1, end: 7 },
                Range { start: 8, end: 12 },
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "1-5\n3-7\n10\n8-12\n9-11\n15";
        let pantry = parse_input(input);

        assert_eq!(
            pantry.ranges,
            vec![
                Range { start: 1, end: 5 },
                Range { start: 3, end: 7 },
                Range { start: 8, end: 12 },
                Range { start: 9, end: 11 },
            ]
        );

        assert_eq!(
            pantry.ingredients,
            vec![
                Ingredient { value: 10 },
                Ingredient { value: 15 },
            ]
        );
    }
}
