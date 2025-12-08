mod days;
mod dsu;
mod grid2d;
mod search;

use std::{fmt::Display, fs, time::Duration};

pub use days::DAYS;

#[derive(Debug, PartialEq)]
pub enum Solution {
    Int(i64),
    String(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Int(x) => write!(f, "{x}"),
            Solution::String(x) => write!(f, "{x}"),
        }
    }
}

pub trait Day: Sync + Send {
    fn part1(&self, input: &str) -> Solution;
    fn part2(&self, input: &str) -> Solution;
}

pub fn load_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);

    fs::read_to_string(&path).expect(&format!("Failed to read input file: {}", path))
}

pub fn fmt_duration(d: Duration) -> String {
    if d.as_secs() >= 1 {
        let s = d.as_secs_f64();
        format!("{:.2} s", s)
    } else if d.as_millis() >= 1 {
        let ms = d.as_secs_f64() * 1000.0;
        format!("{:.2} ms", ms)
    } else if d.as_micros() >= 1 {
        let us = d.as_secs_f64() * 1_000_000.0;
        format!("{:.2} µs", us)
    } else {
        let ns = d.as_secs_f64() * 1_000_000_000.0;
        format!("{:.2} ns", ns)
    }
}
