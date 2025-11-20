mod days;

use std::fs;

pub use days::DAYS;

pub trait Day: Sync + Send {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn load_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);

    fs::read_to_string(&path).expect(&format!("Failed to read input file: {}", path))
}
