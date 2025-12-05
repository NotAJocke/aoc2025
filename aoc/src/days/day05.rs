use std::{cmp::max, ops::RangeInclusive};

use crate::{Day, Solution};

pub struct Day05;

impl Day for Day05 {
    fn part1(&self, input: &str) -> Solution {
        let (ranges, available) = parse_input(input);

        let ranges = merge_ranges(ranges);

        let fresh_count = available
            .iter()
            .filter(|&&i| in_any_range(&ranges, i))
            .count();

        Solution::Int(fresh_count as i64)
    }

    fn part2(&self, input: &str) -> Solution {
        let (ranges, _) = parse_input(input);

        let ranges = merge_ranges(ranges);

        let expanded_count = ranges.iter().map(|r| r.end() - r.start() + 1).sum();

        Solution::Int(expanded_count)
    }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    input
        .trim()
        .split_once("\n\n")
        .map(|(a, b)| {
            (
                a.lines()
                    .map(|range| {
                        range
                            .split_once('-')
                            .map(|(x, y)| x.parse().unwrap()..=y.parse().unwrap())
                            .unwrap()
                    })
                    .collect(),
                b.lines().map(|x| x.parse().unwrap()).collect(),
            )
        })
        .unwrap()
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    ranges.sort_by_key(|r| *r.start());

    let mut merged_ranges = Vec::new();
    let mut ranges_iter = ranges.into_iter();
    let mut current = ranges_iter.next().unwrap();

    for range in ranges_iter {
        if can_merge_ranges(&current, &range) {
            current = *current.start()..=max(*current.end(), *range.end());
        } else {
            merged_ranges.push(current);
            current = range;
        }
    }
    merged_ranges.push(current);

    merged_ranges
}

#[inline]
fn can_merge_ranges(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

fn in_any_range(ranges: &[RangeInclusive<i64>], value: i64) -> bool {
    // Previously wrote my own, didn't know about this method, great learning

    ranges
        .binary_search_by(|r| {
            if value < *r.start() {
                std::cmp::Ordering::Greater
            } else if value > *r.end() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day05::Day05};

    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    const SOLVER: Day05 = Day05;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(3));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(14));
    }
}
