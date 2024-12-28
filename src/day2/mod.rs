pub mod part1;
pub mod part2;

mod common {
    pub fn process_content(content: &str) -> Vec<Vec<i32>> {
        content
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|level| level.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect()
    }

    pub fn is_increasing(vec: &[i32]) -> bool {
        vec.windows(2).all(|window| window[0] < window[1])
    }

    pub fn is_decreasing(vec: &[i32]) -> bool {
        vec.windows(2).all(|window| window[0] > window[1])
    }

    pub fn is_difference_safe(vec: &[i32]) -> bool {
        vec.windows(2)
            .all(|window| (window[0] - window[1]).abs() <= 3)
    }
}
