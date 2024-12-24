use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
};

use crate::utils;

#[derive(Debug)]
struct Input {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

pub fn solve(output: &mut File) {
    let content = fs::read_to_string("src/day5/input.txt").expect("File doesn't exist");

    let mut input = process_content(&content);

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

fn process_content(content: &str) -> Input {
    let (rules, updates) = content.split_once("\r\n\r\n").unwrap();
    Input {
        rules: extract_rules(rules),
        updates: extract_updates(updates),
    }
}

fn extract_rules(content: &str) -> HashMap<u32, HashSet<u32>> {
    content
        .split_whitespace()
        .filter_map(|line| {
            let mut numbers = line
                .split('|')
                .filter_map(|number| number.parse::<u32>().ok());
            Some((numbers.next()?, numbers.next()?))
        })
        .fold(HashMap::new(), |mut map, pair| {
            map.entry(pair.1)
                .or_insert_with(HashSet::new)
                .insert(pair.0);
            map
        })
}

fn extract_updates(content: &str) -> Vec<Vec<u32>> {
    content
        .split_whitespace()
        .map(|line| {
            line.split(',')
                .filter_map(|number| number.parse::<u32>().ok())
                .collect()
        })
        .collect()
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
