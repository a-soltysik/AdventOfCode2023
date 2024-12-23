use std::{env, fs::File};

mod utils;

mod day1;
mod day2;
mod day3;

fn main() {
    let day = env::args().nth(1).and_then(|arg| arg.parse::<u32>().ok());
    let mut file = File::create("output.txt").expect("Error creating file");
    match day {
        Some(1) => {
            day1::part1::solve(&mut file);
            day1::part2::solve(&mut file);
        }
        Some(2) => {
            day2::part1::solve(&mut file);
            day2::part2::solve(&mut file);
        }
        Some(3) => {
            day3::part1::solve(&mut file);
            day3::part2::solve(&mut file);
        }
        Some(x) => println!("Day {x} is not implemented"),
        None => println!("Usage: program <day_number>"),
    }
}
