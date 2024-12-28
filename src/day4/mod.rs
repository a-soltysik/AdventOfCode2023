pub mod part1;
pub mod part2;

mod common {
    pub fn process_content(content: &str) -> Vec<Vec<char>> {
        content
            .split_whitespace()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect()
    }
}
