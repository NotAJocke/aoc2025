use std::ops::Sub;

use crate::{Day, Solution};

pub struct Day07;

impl Day for Day07 {
    fn part1(&self, input: &str) -> Solution {
        let (beam_start, char_grid) = parse_input(input.trim());

        let mut grid = vec![vec![false; char_grid[0].len()]; char_grid.len()];

        grid[0][beam_start] = true;

        let mut splits = 0;
        for i in 0..char_grid.len().sub(1) {
            let next_i = i + 1;

            for (j, &cell) in char_grid[i].iter().enumerate() {
                if cell == '.' {
                    if grid[i][j] {
                        grid[next_i][j] = true;
                    }
                } else if cell == '^' && grid[i][j] {
                    splits += 1;

                    if j > 0 {
                        grid[next_i][j - 1] = true;
                    }

                    if j + 1 < char_grid[i].len() {
                        grid[next_i][j + 1] = true;
                    }
                }
            }
        }

        Solution::Int(splits)
    }

    fn part2(&self, input: &str) -> Solution {
        let (beam_start, char_grid) = parse_input(input.trim());

        let mut grid = vec![vec![0; char_grid[0].len()]; char_grid.len()];

        grid[0][beam_start] = 1;

        for i in 0..char_grid.len().sub(1) {
            let next_i = i + 1;

            for (j, &cell) in char_grid[i].iter().enumerate() {
                let current_count = grid[i][j];
                if current_count == 0 {
                    continue;
                }

                if cell == '.' {
                    grid[next_i][j] += current_count;
                } else if cell == '^' {
                    if j > 0 {
                        grid[next_i][j - 1] += current_count;
                    }
                    if j + 1 < char_grid[i].len() {
                        grid[next_i][j + 1] += current_count;
                    }
                }
            }
        }

        let total_timelines: u64 = grid[grid.len() - 1].iter().sum();
        Solution::Int(total_timelines as i64)
    }
}

fn parse_input(input: &str) -> (usize, Vec<Vec<char>>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let rest = lines.into_iter();

    (
        first_line.find('S').unwrap(),
        rest.map(|x| x.chars().collect()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day07::Day07};

    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    const SOLVER: Day07 = Day07;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(21));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(40));
    }
}
