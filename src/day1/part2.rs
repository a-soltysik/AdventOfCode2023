use std::{
    collections::HashMap,
    fs::{self, File},
};

use crate::utils;

pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day1/input.txt").expect("File doesn't exist");

    let (vec, map) = process_content(content.as_str());

    utils::write_output(output, calculate_difference(&vec, &map));
}

fn process_content(content: &str) -> (Vec<i32>, HashMap<i32, i32>) {
    content
        .split_whitespace()
        .map(|word| word.parse::<i32>())
        .enumerate()
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut vec, mut map), (index, value)| {
                if let Ok(val) = value {
                    if index % 2 == 0 {
                        vec.push(val);
                    } else {
                        *map.entry(val).or_insert(0) += 1;
                    }
                }
                (vec, map)
            },
        )
}

fn calculate_difference(vec: &[i32], map: &HashMap<i32, i32>) -> i32 {
    vec.iter().fold(0, |acc, &id| {
        acc + map.get(&id).map_or(0, |&count| id * count)
    })
}
