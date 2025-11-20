use crate::Day;

/// This is a test day (day1 from 2024)
/// https://adventofcode.com/2024/day/1
pub struct Day00;
impl Day for Day00 {
    fn part1(&self, input: &str) -> String {
        let (mut a, mut b) =
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
                });

        a.sort();
        b.sort();

        let x: i32 = a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum();

        x.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "0".to_string()
    }
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

    #[test]
    fn test_part1() {
        let solver = Day00;
        assert_eq!(solver.part1(TEST).as_str(), "11");
    }
}
