use std::cmp::Ordering;

use super::{merge_ranges, parse_input};

pub fn solve(input: &str) -> u64 {
    let pantry = parse_input(input);
    let merged = merge_ranges(pantry.ranges);

    let mut fresh_ingredients_count = 0;
    
    for ingredient in pantry.ingredients.iter() {
        let result_index = merged.binary_search_by(|range| {
            if ingredient.value < range.start {
                Ordering::Greater
            } else if ingredient.value > range.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        if result_index.is_ok() {
            fresh_ingredients_count += 1;
        }
    }

    fresh_ingredients_count
}
