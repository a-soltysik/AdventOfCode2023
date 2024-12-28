use std::fs::{self, File};

use crate::utils;

use super::common;

/// Count how many lines are strictly increasing or decreasing
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day2/input.txt").expect("File doesn't exist");

    let matrix = common::process_content(&content);

    utils::write_output(
        output,
        matrix
            .iter()
            .filter(|line| {
                common::is_difference_safe(line)
                    && (common::is_increasing(line) || common::is_decreasing(line))
            })
            .count(),
    );
}
