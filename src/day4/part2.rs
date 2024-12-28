use std::fs::{self, File};

use crate::utils;

use super::common;

pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day4/input.txt").expect("File doesn't exist");

    let grid = common::process_content(&content);

    utils::write_output(output, iterate_windows(&grid));
}

fn iterate_windows(grid: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for i in 0..=grid.len() - 3 {
        for j in 0..=grid[i].len() - 3 {
            let mut window = Vec::with_capacity(9);
            for x in 0..3 {
                for y in 0..3 {
                    window.push(grid[i + x][j + y]);
                }
            }
            if is_xmas(&window) {
                result += 1;
            }
        }
    }
    result
}
fn is_xmas(window: &[char]) -> bool {
    if window[4] != 'A' {
        return false;
    }
    if (window[0] as i32 - window[8] as i32).abs() == ('M' as i32 - 'S' as i32).abs()
        && (window[2] as i32 - window[6] as i32).abs() == ('M' as i32 - 'S' as i32).abs()
    {
        return true;
    }
    return false;
}
