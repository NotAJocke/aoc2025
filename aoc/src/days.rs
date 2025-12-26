use std::{collections::HashMap, sync::LazyLock};

use crate::Day;

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub static DAYS: LazyLock<HashMap<u8, Box<dyn Day + Send + Sync>>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(0, Box::new(crate::days::day00::Day00) as Box<_>);
    map.insert(1, Box::new(crate::days::day01::Day01) as Box<_>);
    map.insert(2, Box::new(crate::days::day02::Day02) as Box<_>);
    map.insert(3, Box::new(crate::days::day03::Day03) as Box<_>);
    map.insert(4, Box::new(crate::days::day04::Day04) as Box<_>);
    map.insert(5, Box::new(crate::days::day05::Day05) as Box<_>);
    map.insert(6, Box::new(crate::days::day06::Day06) as Box<_>);
    map.insert(7, Box::new(crate::days::day07::Day07) as Box<_>);
    map.insert(8, Box::new(crate::days::day08::Day08) as Box<_>);
    map.insert(9, Box::new(crate::days::day09::Day09) as Box<_>);
    map.insert(10, Box::new(crate::days::day10::Day10) as Box<_>);
    map.insert(11, Box::new(crate::days::day11::Day11) as Box<_>);
    map.insert(12, Box::new(crate::days::day12::Day12) as Box<_>);

    map
});
