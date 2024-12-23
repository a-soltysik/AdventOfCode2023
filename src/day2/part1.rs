use std::fs::{self, File};

use crate::utils;

pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day2/input.txt").expect("File doesn't exist");

    let matrix = process_content(&content);

    utils::write_output(
        output,
        matrix
            .iter()
            .filter(|line| is_difference_safe(line) && (is_increasing(line) || is_decreasing(line)))
            .count(),
    );
}

fn process_content(content: &str) -> Vec<Vec<i32>> {
    content
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().ok())
                .flatten()
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_increasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|window| window[0] < window[1])
}

fn is_decreasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|window| window[0] > window[1])
}

fn is_difference_safe(vec: &[i32]) -> bool {
    vec.windows(2)
        .all(|window| (window[0] - window[1]).abs() <= 3)
}
