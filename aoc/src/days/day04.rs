use std::collections::VecDeque;

use rustc_hash::FxHashMap;

use crate::{Day, Solution, grid2d::Grid2D};

pub struct Day04;

impl Day for Day04 {
    fn part1(&self, input: &str) -> Solution {
        let grid: Grid2D<char> = Grid2D::from(input.trim());

        let count = grid
            .iter()
            .filter(|&((x, y), &cell)| {
                if cell != '@' {
                    return false;
                }
                grid.all_neighbors(x, y)
                    .iter()
                    .filter(|&&(_, &neighbor)| neighbor == '@')
                    .take(4)
                    .count()
                    < 4
            })
            .count();

        Solution::Int(count as i64)
    }

    fn part2(&self, input: &str) -> Solution {
        const NEIGHBORS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (0, -1),
        ];
        let grid: Grid2D<char> = Grid2D::from(input.trim());

        let mut rolls: FxHashMap<(usize, usize), usize> = FxHashMap::default();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        grid.iter().for_each(|((x, y), &cell)| {
            if cell == '@' {
                let neighbors_count = grid
                    .all_neighbors(x, y)
                    .iter()
                    .filter(|&&(_, &neighbor)| neighbor == '@')
                    .count();

                if neighbors_count < 4 {
                    queue.push_back((x, y));
                } else {
                    rolls.insert((x, y), neighbors_count);
                }
            }
        });

        let mut count = 0i64;
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            count += 1;

            NEIGHBORS
                .iter()
                .filter_map(|&(dx, dy)| {
                    let nx = x.checked_add_signed(dx)?;
                    let ny = y.checked_add_signed(dy)?;

                    Some((nx, ny))
                })
                .for_each(|n| {
                    let Some(&neighbors_count) = rolls.get(&n) else {
                        return;
                    };

                    if neighbors_count - 1 < 4 {
                        rolls.remove(&n);
                        queue.push_back(n);
                    } else {
                        rolls.entry(n).and_modify(|x| *x -= 1);
                    }
                });
        }

        Solution::Int(count)
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
