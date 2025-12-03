use crate::{Day, Solution};

pub struct Day03;

impl Day for Day03 {
    fn part1(&self, input: &str) -> Solution {
        let grid = parse_input(input);

        let total: i64 = grid
            .into_iter()
            .map(|line| {
                let mut max_left = line[0];
                let mut best = 0i64;

                for &n in &line[1..] {
                    let value = (max_left * 10 + n) as i64;
                    if value > best {
                        best = value;
                    }

                    if n > max_left {
                        max_left = n;
                    }
                }
                best
            })
            .sum();

        Solution::Int(total)
    }

    fn part2(&self, input: &str) -> Solution {
        let grid = parse_input(input);

        let total: i64 = grid
            .into_iter()
            .map(|line| {
                let n = line.len();
                let k = 12;
                let mut stack: Vec<u32> = Vec::with_capacity(k);

                for (i, &digit) in line.iter().enumerate() {
                    while !stack.is_empty()
                        && digit > *stack.last().unwrap()
                        && (stack.len() - 1 + (n - i)) >= k
                    {
                        stack.pop();
                    }

                    if stack.len() < k {
                        stack.push(digit);
                    }
                }

                let mut result: i64 = 0;
                for &digit in &stack {
                    result = result * 10 + digit as i64;
                }
                result
            })
            .sum();

        Solution::Int(total)
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day03::Day03};

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    const SOLVER: Day03 = Day03;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(357));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(3121910778619))
    }
}
