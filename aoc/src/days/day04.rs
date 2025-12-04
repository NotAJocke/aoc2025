use crate::{Day, Solution, grid2d::Grid2D};

pub struct Day04;

impl Day for Day04 {
    fn part1(&self, input: &str) -> Solution {
        let grid: Grid2D<char> = Grid2D::from(input.trim());

        let count = grid
            .into_iter()
            .filter(|&((x, y), &cell)| {
                if cell != '@' {
                    return false;
                }
                grid.all_neighbors(x, y)
                    .into_iter()
                    .filter(|&(_, neighbor)| neighbor == '@')
                    .take(4)
                    .count()
                    < 4
            })
            .count();

        Solution::Int(count as i64)
    }

    fn part2(&self, input: &str) -> Solution {
        let mut grid: Grid2D<char> = Grid2D::from(input.trim());

        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        let mut total_count = 0i64;

        loop {
            for &(x, y) in &to_remove {
                *grid.get_mut(x, y) = 'x';
            }
            to_remove.clear();

            grid.into_iter()
                .filter(|&((x, y), &cell)| {
                    if cell != '@' {
                        return false;
                    }
                    grid.all_neighbors(x, y)
                        .into_iter()
                        .filter(|&(_, neighbor)| neighbor == '@')
                        .take(4)
                        .count()
                        < 4
                })
                .for_each(|((x, y), _)| {
                    to_remove.push((x, y));
                });

            total_count += to_remove.len() as i64;

            if to_remove.is_empty() {
                break;
            }
        }

        Solution::Int(total_count)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day04::Day04};

    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    const SOLVER: Day04 = Day04;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(13));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(43));
    }
}
