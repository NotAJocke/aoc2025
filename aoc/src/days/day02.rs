use rayon::prelude::*;
use std::ops::RangeInclusive;

use crate::Day;

pub struct Day02;

impl Day for Day02 {
    fn part1(&self, input: &str) -> String {
        parse_ranges(input)
            .flat_map(|r| r)
            .filter(|&x| is_invalid(x))
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse_ranges(input)
            .flat_map(|r| r)
            .filter(|&x| is_invalid2(x))
            .sum::<i64>()
            .to_string()
    }
}

fn parse_ranges(input: &str) -> impl ParallelIterator<Item = RangeInclusive<i64>> {
    input.trim().par_split(',').map(|range| {
        let (a, b) = range.split_once('-').unwrap();
        let (a, b): (i64, i64) = (a.parse().unwrap(), b.parse().unwrap());

        a..=b
    })
}

fn is_invalid(x: i64) -> bool {
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(x);

    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() / 2;
    &s[..mid] == &s[mid..]
}

fn is_invalid2(x: i64) -> bool {
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(x);

    let bytes = s.as_bytes();
    let n = bytes.len();

    let z = crate::search::z_algorithm(bytes);

    // Try every prefix length that divides n
    for k in 1..=n / 2 {
        if n % k == 0 && z[k] == n - k {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{
        Day,
        days::day02::{Day02, is_invalid, is_invalid2},
    };

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    const SOLVER: Day02 = Day02;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), "4174379265");
    }

    #[test]
    fn validity() {
        let v1 = 123;
        let v2 = 894;
        let iv1 = 55;
        let iv2 = 6464;
        let iv3 = 123123;

        assert!(!is_invalid(v1));
        assert!(!is_invalid(v2));
        assert!(is_invalid(iv1));
        assert!(is_invalid(iv2));
        assert!(is_invalid(iv3));
    }

    #[test]
    fn validity2() {
        let v1 = 123;
        let v2 = 894;
        let iv1 = 12341234;
        let iv2 = 123123123;
        let iv3 = 1212121212;
        let iv4 = 1111111;
        let iv5 = 111;

        assert!(!is_invalid2(v1));
        assert!(!is_invalid2(v2));
        assert!(is_invalid2(iv1));
        assert!(is_invalid2(iv2));
        assert!(is_invalid2(iv3));
        assert!(is_invalid2(iv4));
        assert!(is_invalid2(iv5));
    }
}
