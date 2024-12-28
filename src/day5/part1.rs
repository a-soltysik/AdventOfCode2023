use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
};

use crate::utils;

use super::common;

/// Map each page number to all page numbers that occur before it. Then, for each number in an update, check if all consecutive numbers are not included in the map
pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day5/input.txt").expect("File doesn't exist");

    let input = common::process_content(&content);

    utils::write_output(
        output,
        input
            .updates
            .iter()
            .filter(|update| is_update_correct(&update, &input.rules))
            .filter_map(|update| update.get(update.len() / 2))
            .sum::<u32>(),
    );
}

fn is_update_correct(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    update.iter().enumerate().all(|(id, page)| {
        update.iter().skip(id + 1).all(|to_check| {
            rules
                .get(page)
                .map_or(true, |next| !next.contains(to_check))
        })
    })
}
