use std::fs::{self, File};

use regex::Regex;

use crate::utils;

pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day3/input.txt").expect("File doesn't exist");

    let instructions = process_content(&content);

    utils::write_output(
        output,
        instructions.iter().map(|pair| pair.0 * pair.1).sum::<u32>(),
    );
}

fn process_content(content: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(content)
        .map(|capture| (capture[1].parse().unwrap(), capture[2].parse().unwrap()))
        .collect()
}
