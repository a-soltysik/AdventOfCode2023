use std::fs::{self, File};

use crate::utils;

pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day2/input.txt").expect("File doesn't exist");

    let matrix = process_content(&content);

    utils::write_output(
        output,
        matrix.iter().filter(|line| check_line(&line)).count(),
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

fn check_line(vec: &[i32]) -> bool {
    (0..vec.len()).any(|position| can_be_fixed(vec, position))
}

fn can_be_fixed(vec: &[i32], position: usize) -> bool {
    let filtered = [&vec[..position], &vec[position + 1..]].concat();

    is_difference_safe(&filtered) && (is_decreasing(&filtered) || is_increasing(&filtered))
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
