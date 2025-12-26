use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use crate::{Day, Solution};

pub struct Day12;

impl Day for Day12 {
    fn part1(&self, input: &str) -> Solution {
        let (shapes, queries) = parse_input(input);
        let variations = generate_variations(shapes);

        let shape_areas: std::collections::HashMap<usize, usize> = variations
            .iter()
            .map(|(&id, vars)| {
                (
                    id,
                    vars[0].rows.iter().map(|r| r.count_ones() as usize).sum(),
                )
            })
            .collect();

        let mut valid_count = 0;
        for (width, height, counts) in queries {
            let mut items = Vec::new();
            for (sid, &count) in counts.iter().enumerate() {
                for _ in 0..count {
                    items.push(Item {
                        id: sid,
                        area: shape_areas[&sid],
                        placed_r: 0,
                        placed_c: 0,
                    });
                }
            }

            if items.is_empty() {
                valid_count += 1;
                continue;
            }

            items.sort_by_key(|it| (-(it.area as i32), it.id));
            let total_area: usize = items.iter().map(|i| i.area).sum();
            if total_area > width * height {
                continue;
            }

            let min_area = items.last().unwrap().area;
            let mut grid = vec![0u128; height];

            if solve_recursive(
                &mut grid,
                &mut items,
                0,
                width,
                height,
                &variations,
                min_area,
            ) {
                valid_count += 1;
            }
        }

        Solution::Int(valid_count as i64)
    }

    fn part2(&self, _input: &str) -> Solution {
        Solution::Int(0)
    }
}

#[derive(Clone, Debug)]
struct Variation {
    h: usize,
    w: usize,
    rows: Vec<u128>,
}

#[derive(Clone, Debug)]
struct Item {
    id: usize,
    area: usize,
    placed_r: usize,
    placed_c: usize,
}

fn parse_input(
    input: &str,
) -> (
    std::collections::HashMap<usize, Vec<(u128, usize)>>,
    Vec<(usize, usize, Vec<usize>)>,
) {
    let mut shapes = std::collections::HashMap::new();
    let mut grids = Vec::new();
    let mut current_id = 0;

    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<usize> = parts[0].split('x').map(|s| s.parse().unwrap()).collect();
            let counts: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            grids.append(&mut vec![(dims[0], dims[1], counts)]);
        } else if line.ends_with(':') {
            current_id = line[..line.len() - 1].parse().unwrap();
            shapes.insert(current_id, Vec::new());
        } else {
            let val = line.chars().enumerate().fold(0u128, |acc, (j, c)| {
                if c == '#' {
                    acc | (1 << (line.len() - 1 - j))
                } else {
                    acc
                }
            });
            shapes.get_mut(&current_id).unwrap().push((val, line.len()));
        }
        i += 1;
    }
    (shapes, grids)
}

fn generate_variations(
    base_shapes: std::collections::HashMap<usize, Vec<(u128, usize)>>,
) -> std::collections::HashMap<usize, Vec<Variation>> {
    let mut variations = std::collections::HashMap::new();

    for (sid, rows) in base_shapes {
        let max_w = rows.iter().map(|r| r.1).max().unwrap_or(0);
        let mut matrix = Vec::new();
        for (val, w) in rows {
            let mut row_bits = Vec::new();
            for j in 0..max_w {
                row_bits.push((val >> (w - 1 - j)) & 1 == 1);
            }
            matrix.push(row_bits);
        }

        let mut seen_hashes = FxHashSet::default();
        let mut shape_vars = Vec::new();
        let mut current = matrix;

        for _ in 0..2 {
            for _ in 0..4 {
                let (mut min_r, mut max_r) = (current.len(), 0);
                let (mut min_c, mut max_c) = (current[0].len(), 0);
                let mut has_bits = false;

                for r in 0..current.len() {
                    for c in 0..current[0].len() {
                        if current[r][c] {
                            has_bits = true;
                            min_r = min_r.min(r);
                            max_r = max_r.max(r);
                            min_c = min_c.min(c);
                            max_c = max_c.max(c);
                        }
                    }
                }

                if has_bits {
                    let mut int_rows = Vec::new();
                    for r in min_r..=max_r {
                        let mut r_val = 0u128;
                        for c in min_c..=max_c {
                            r_val = (r_val << 1) | if current[r][c] { 1 } else { 0 };
                        }
                        int_rows.push(r_val);
                    }

                    if seen_hashes.insert(int_rows.clone()) {
                        shape_vars.push(Variation {
                            h: max_r - min_r + 1,
                            w: max_c - min_c + 1,
                            rows: int_rows,
                        });
                    }
                }

                let h = current.len();
                let w = current[0].len();
                let mut next = vec![vec![false; h]; w];
                for r in 0..h {
                    for c in 0..w {
                        next[c][h - 1 - r] = current[r][c];
                    }
                }
                current = next;
            }
            current.reverse();
        }
        shape_vars.sort_by_key(|v| -(v.h as i32));
        variations.insert(sid, shape_vars);
    }
    variations
}

fn is_space_sufficient(
    grid: &[u128],
    width: usize,
    height: usize,
    required_area: usize,
    min_item_area: usize,
) -> bool {
    let used_area: usize = grid.iter().map(|row| row.count_ones() as usize).sum();
    let free_area = (width * height).saturating_sub(used_area);
    if free_area < required_area {
        return false;
    }
    if free_area > required_area + 20 {
        return true;
    }

    let mut visited = vec![vec![false; width]; height];
    let mut usable_free_area = 0;

    for r in 0..height {
        for c in 0..width {
            let is_occupied = (grid[r] >> (width - 1 - c)) & 1 == 1;
            if !is_occupied && !visited[r][c] {
                let mut island_size = 0;
                let mut q = VecDeque::new();
                q.push_back((r, c));
                visited[r][c] = true;

                while let Some((curr_r, curr_c)) = q.pop_front() {
                    island_size += 1;
                    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let nr = curr_r as i32 + dr;
                        let nc = curr_c as i32 + dc;
                        if nr >= 0 && nr < height as i32 && nc >= 0 && nc < width as i32 {
                            let (nr, nc) = (nr as usize, nc as usize);
                            let n_occupied = (grid[nr] >> (width - 1 - nc)) & 1 == 1;
                            if !n_occupied && !visited[nr][nc] {
                                visited[nr][nc] = true;
                                q.push_back((nr, nc));
                            }
                        }
                    }
                }
                if island_size >= min_item_area {
                    usable_free_area += island_size;
                }
                if usable_free_area >= required_area {
                    return true;
                }
            }
        }
    }
    usable_free_area >= required_area
}

fn solve_recursive(
    grid: &mut Vec<u128>,
    items: &mut Vec<Item>,
    item_idx: usize,
    width: usize,
    height: usize,
    variations: &std::collections::HashMap<usize, Vec<Variation>>,
    min_global_area: usize,
) -> bool {
    if item_idx == items.len() {
        return true;
    }

    let remaining_area: usize = items[item_idx..].iter().map(|it| it.area).sum();
    if !is_space_sufficient(grid, width, height, remaining_area, min_global_area) {
        return false;
    }

    let sid = items[item_idx].id;
    let mut start_r = 0;
    let mut start_c = 0;

    if item_idx > 0 && items[item_idx - 1].id == sid {
        start_r = items[item_idx - 1].placed_r;
        start_c = items[item_idx - 1].placed_c;
    }

    let vars = &variations[&sid];
    for var in vars {
        if var.h > height || var.w > width {
            continue;
        }
        for r in start_r..=(height - var.h) {
            let c_begin = if r == start_r { start_c } else { 0 };
            for c in c_begin..=(width - var.w) {
                let shift = width - c - var.w;
                let mut fits = true;
                for i in 0..var.h {
                    if (grid[r + i] & (var.rows[i] << shift)) != 0 {
                        fits = false;
                        break;
                    }
                }

                if fits {
                    for i in 0..var.h {
                        grid[r + i] |= var.rows[i] << shift;
                    }
                    items[item_idx].placed_r = r;
                    items[item_idx].placed_c = c;

                    if solve_recursive(
                        grid,
                        items,
                        item_idx + 1,
                        width,
                        height,
                        variations,
                        min_global_area,
                    ) {
                        return true;
                    }

                    for i in 0..var.h {
                        grid[r + i] ^= var.rows[i] << shift;
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day12::Day12};

    const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2\
";

    const SOLVER: Day12 = Day12;

    #[test]
    fn test_part1() {
        let result = SOLVER.part1(TEST);
        assert_eq!(result, Solution::Int(2))
    }
}
