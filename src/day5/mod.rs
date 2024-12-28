pub mod part1;
pub mod part2;

mod common {
    use std::collections::{HashMap, HashSet};

    pub struct Input {
        pub rules: HashMap<u32, HashSet<u32>>,
        pub updates: Vec<Vec<u32>>,
    }

    pub fn process_content(content: &str) -> Input {
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
}
