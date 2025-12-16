use std::collections::HashMap;

use crate::day_7::parse_input;

pub fn solve(input: &str) -> i64 {
    let mut data = parse_input(input);
    let start_line = data.remove(0);
    let start_index = start_line.iter().position(|char| *char == 'S').expect("S to exist");
    let mut split_map: HashMap<usize, i64> = HashMap::new();
    split_map.insert(start_index, 1);

    for line in data.iter() {
        let mut tmp_split_map: HashMap<usize, i64> = HashMap::new();

        for (&index, &count) in split_map.iter() {
            let char = line[index];

            match char {
                '^' => {
                    if index > 0 {
                        *tmp_split_map.entry(index - 1).or_insert(0) += count;
                    }
                    *tmp_split_map.entry(index + 1).or_insert(0) += count;
                },
                '.' => {
                    *tmp_split_map.entry(index).or_insert(0) += count;
                },
                _ => continue
            }
        }

        split_map = tmp_split_map;
    }

    split_map.values().sum::<i64>()
}
