use std::collections::HashMap;

use crate::{Day, Solution};

/// This is a test day (day1 from 2024)
/// https://adventofcode.com/2024/day/1
pub struct Day00;
impl Day for Day00 {
    fn part1(&self, input: &str) -> Solution {
        let (mut a, mut b) = parse_lists(input);

        a.sort();
        b.sort();

        Solution::Int(a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum::<i32>() as i64)
    }

    fn part2(&self, input: &str) -> Solution {
        let (a, b) = parse_lists(input);

        let freq: HashMap<i32, i32> = b.iter().fold(HashMap::new(), |mut m, &x| {
            *m.entry(x).or_default() += 1;
            m
        });

        Solution::Int(
            a.iter()
                .map(|&x| x * freq.get(&x).copied().unwrap_or(0))
                .sum::<i32>() as i64,
        )
    }
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut inputs = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

            (inputs.next().unwrap(), inputs.next().unwrap())
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day00::Day00};

    const TEST: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    const SOLVER: Day00 = Day00;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(11));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(31));
    }
}
