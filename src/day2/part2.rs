use std::fs::{self, File};

use crate::utils;

use super::common;

/// Brute force solution: try removing each number from the line to see if the line can be fixed.
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day2/input.txt").expect("File doesn't exist");

    let matrix = common::process_content(&content);

    utils::write_output(
        output,
        matrix.iter().filter(|line| check_line(&line)).count(),
    );
}

fn check_line(vec: &[i32]) -> bool {
    (0..vec.len()).any(|position| can_be_fixed(vec, position))
}

fn can_be_fixed(vec: &[i32], position: usize) -> bool {
    let filtered = [&vec[..position], &vec[position + 1..]].concat();

    common::is_difference_safe(&filtered)
        && (common::is_decreasing(&filtered) || common::is_increasing(&filtered))
}
