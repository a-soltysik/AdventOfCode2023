use std::fs::{self, File};

use regex::Regex;

use crate::utils;

enum RegexResult {
    Multiplication(usize, (u32, u32)),
    Do(usize),
    Dont(usize),
}

/// Find all mul(x, y), do() and don't(), sort them by position in string and calculate all mul expression, expect ones between don't() and do()
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day3/input.txt").expect("File doesn't exist");

    let mut instructions = process_content(&content);

    let mut dont_range = false;
    instructions.retain(|item| match item {
        RegexResult::Do(_) => {
            dont_range = false;
            true
        }
        RegexResult::Dont(_) => {
            dont_range = true;
            true
        }
        RegexResult::Multiplication(_, _) => !dont_range,
    });

    utils::write_output(
        output,
        instructions
            .iter()
            .map(|pair| match pair {
                RegexResult::Multiplication(_, values) => values.0 * values.1,
                RegexResult::Do(_) => 0,
                RegexResult::Dont(_) => 0,
            })
            .sum::<u32>(),
    );
}

fn process_content(content: &str) -> Vec<RegexResult> {
    let multiplication = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let dont = Regex::new(r"don't\(\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();

    let mut result = Vec::new();

    result.extend(multiplication.captures_iter(content).filter_map(|capture| {
        Some(RegexResult::Multiplication(
            capture.get(0)?.start(),
            (capture[1].parse().ok()?, capture[2].parse().ok()?),
        ))
    }));

    result.extend(
        dont.captures_iter(content)
            .filter_map(|capture| Some(RegexResult::Dont(capture.get(0)?.start()))),
    );

    result.extend(
        do_regex
            .captures_iter(content)
            .filter_map(|capture| Some(RegexResult::Do(capture.get(0)?.start()))),
    );

    result.sort_by_key(|value| match value {
        RegexResult::Multiplication(position, _) => *position,
        RegexResult::Do(position) => *position,
        RegexResult::Dont(position) => *position,
    });

    result
}
