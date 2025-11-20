use std::{collections::HashMap, sync::LazyLock};

use crate::Day;

mod day00;

pub static DAYS: LazyLock<HashMap<u8, Box<dyn Day + Send + Sync>>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(0, Box::new(crate::days::day00::Day00) as Box<_>);

    map
});
