use std::{collections::HashMap, sync::LazyLock};

use crate::Day;

mod day00;
mod day01;
mod day02;

pub static DAYS: LazyLock<HashMap<u8, Box<dyn Day + Send + Sync>>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(0, Box::new(crate::days::day00::Day00) as Box<_>);
    map.insert(1, Box::new(crate::days::day01::Day01) as Box<_>);
    map.insert(2, Box::new(crate::days::day02::Day02) as Box<_>);

    map
});
