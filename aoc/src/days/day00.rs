use crate::Day;

/// This is a test day (day1 from 2024)
/// https://adventofcode.com/2024/day/1
pub struct Day00;
impl Day for Day00 {
    fn part1(&self, input: &str) -> String {
        let (mut a, mut b) = parse_lists(input);

        a.sort();
        b.sort();

        a.iter()
            .zip(b)
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (a, b) = parse_lists(input);

        a.iter()
            .map(|&x| x as usize * b.iter().filter(|&&y| y == x).count())
            .sum::<usize>()
            .to_string()
    }
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut a, mut b), line| {
            let inputs: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            a.push(inputs[0]);
            b.push(inputs[1]);

            (a, b)
        })
}

#[cfg(test)]
mod tests {
    use crate::{Day, days::day00::Day00};

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
        assert_eq!(SOLVER.part1(TEST).as_str(), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST).as_str(), "31");
    }
}
