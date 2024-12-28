use std::fs::{self, File};

use crate::utils;

/// Sort the left and right columns, then calculate the difference between the values at the corresponding positions in both columns
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day1/input.txt").expect("File doesn't exist");

    let (mut vec1, mut vec2) = process_content(&content);

    vec1.sort();
    vec2.sort();

    utils::write_output(output, calculate_difference(&vec1, &vec2));
}

fn process_content(content: &str) -> (Vec<i32>, Vec<i32>) {
    content
        .split_whitespace()
        .map(|word| word.parse::<i32>())
        .enumerate()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut v1, mut v2), (index, value)| {
                match value {
                    Ok(val) => {
                        if index % 2 == 0 {
                            v1.push(val);
                        } else {
                            v2.push(val);
                        }
                    }
                    Err(_) => {}
                }
                (v1, v2)
            },
        )
}

fn calculate_difference(vec1: &[i32], vec2: &[i32]) -> i32 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(id1, id2)| (id1 - id2).abs())
        .sum()
}
