use std::fs::{self, File};

use crate::utils;

use super::common;

// Count all "XMAS" substrings by looking for it in all directions for a given starting point
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day4/input.txt").expect("File doesn't exist");

    let grid = common::process_content(&content);

    let mut result = 0;
    for i in 0..grid.len() as isize {
        result += check_direction((0, i), &grid, |(y, x)| (y + 1, x), grid.len() as isize);
        result += check_direction((i, 0), &grid, |(y, x)| (y, x + 1), grid.len() as isize);
    }

    for i in 0..grid.len() as isize {
        result += check_direction((0, i), &grid, |(y, x)| (y + 1, x + 1), grid.len() as isize);
        result += check_direction(
            (i + 1, 0),
            &grid,
            |(y, x)| (y + 1, x + 1),
            grid.len() as isize,
        );

        result += check_direction(
            (0, grid.len() as isize - 1 - i),
            &grid,
            |(y, x)| (y + 1, x - 1),
            grid.len() as isize,
        );
        result += check_direction(
            (i + 1, grid.len() as isize - 1),
            &grid,
            |(y, x)| (y + 1, x - 1),
            grid.len() as isize,
        );
    }

    utils::write_output(output, result);
}

fn check_direction(
    mut start: (isize, isize),
    grid: &Vec<Vec<char>>,
    direction: fn((isize, isize)) -> (isize, isize),
    range: isize,
) -> u32 {
    let mut result = 0;
    let mut word = String::new();

    while start.0 < range && start.0 >= 0 && start.1 < range && start.1 >= 0 {
        word.push(grid[start.0 as usize][start.1 as usize]);
        if word.len() > 4 {
            word.drain(..1);
        }

        if word == "XMAS" || word == "SAMX" {
            result += 1;
        }
        start = direction(start);
    }
    result
}
