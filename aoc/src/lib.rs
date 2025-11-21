mod days;

use std::{fs, time::Duration};

pub use days::DAYS;

pub trait Day: Sync + Send {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn load_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);

    fs::read_to_string(&path).expect(&format!("Failed to read input file: {}", path))
}

pub fn fmt_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{} s", d.as_secs())
    } else if d.as_millis() > 0 {
        format!("{} ms", d.as_millis())
    } else if d.as_micros() > 0 {
        format!("{} µs", d.as_micros())
    } else {
        format!("{} ns", d.as_nanos())
    }
}
