use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
};

use super::common;
use crate::utils;

/// Map each page number to all page numbers that occur before it. Then, if a number in an update occurs before any of the values in the map, swap these numbers in the update
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day5/input.txt").expect("File doesn't exist");

    let mut input = common::process_content(&content);

    utils::write_output(
        output,
        input
            .updates
            .iter_mut()
            .filter_map(|update| {
                if fix_update(update, &input.rules) {
                    update.get(update.len() / 2)
                } else {
                    None
                }
            })
            .sum::<u32>(),
    );
}

fn fix_update(update: &mut [u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut result = false;
    for i in 0..update.len() {
        if let Some(next) = rules.get(&update[i]) {
            for j in i + 1..update.len() {
                if next.contains(&update[j]) {
                    update.swap(i, j);
                    result = true;
                }
            }
        }
    }
    result
}
