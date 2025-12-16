use std::collections::HashSet;

use crate::day_7::parse_input;

pub fn solve(input: &str) -> usize {
    let mut data = parse_input(input);
    let start_line = data.remove(0);
    let start_index = start_line.iter().position(|char| *char == 'S').expect("S to exist");
    let mut split_set = HashSet::new();
    split_set.insert(start_index);
    let mut total = 0;

    for line in data.iter() {
        let mut new_split_set = HashSet::new();

        for &index in split_set.iter() {
            let char = line[index];

            match char {
                '^' => {
                    total += 1;

                    if index > 0 {
                        new_split_set.insert(index - 1);
                    }
                    if index + 1 < line.len() {
                        new_split_set.insert(index + 1);
                    }
                },
                '.' => {
                    new_split_set.insert(index);
                },
                _ => continue
            }
        }

        split_set = new_split_set;
    }
    
    total
}
