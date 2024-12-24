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

    let input = process_content(&content);

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

fn is_update_correct(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    update.iter().enumerate().all(|(id, page)| {
        update.iter().skip(id + 1).all(|to_check| {
            rules
                .get(page)
                .map_or(true, |next| !next.contains(to_check))
        })
    })
}
