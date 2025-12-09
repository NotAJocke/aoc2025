use itertools::Itertools;

use crate::{Day, Solution};

pub struct Day09;

impl Day for Day09 {
    fn part1(&self, input: &str) -> Solution {
        let mut tiles = parse_input(input);

        tiles.sort_by_key(|tile| tile.0);

        let combinations = tiles.into_iter().combinations(2);

        let mut max = 0;
        for combination in combinations {
            let a = combination[0];
            let b = combination[1];

            let dx = b.0 - a.0;
            let dy = (b.1 - a.1).abs();

            let area = (dx + 1) * (dy + 1);
            if area > max {
                max = area;
            }
        }

        Solution::Int(max)
    }

    fn part2(&self, input: &str) -> Solution {
        let tiles = parse_input(input);
        let edges = build_polygon(&tiles);

        let combinations = tiles.iter().combinations(2);

        let mut max_area = 0;
        for combination in combinations {
            let a = combination[0];
            let b = combination[1];

            let x_min = a.0.min(b.0);
            let x_max = a.0.max(b.0);
            let y_min = a.1.min(b.1);
            let y_max = a.1.max(b.1);

            let corner1 = (x_min, y_max);
            let corner2 = (x_max, y_min);

            if !point_in_polygon(corner1, &edges) {
                continue;
            }
            if !point_in_polygon(corner2, &edges) {
                continue;
            }

            if rectangle_has_interior_intersection((x_min, x_max, y_min, y_max), &edges) {
                continue;
            }

            let area = (x_max - x_min + 1) * (y_max - y_min + 1);
            if area > max_area {
                max_area = area;
            }
        }

        Solution::Int(max_area)
    }
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut coords = line.split(",").map(|x| x.parse().unwrap());

            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Edge {
    Horizontal { y: i64, x1: i64, x2: i64 },
    Vertical { x: i64, y1: i64, y2: i64 },
}

impl Edge {
    fn from_points(p1: (i64, i64), p2: (i64, i64)) -> Self {
        if p1.0 == p2.0 {
            Edge::Vertical {
                x: p1.0,
                y1: p1.1.min(p2.1),
                y2: p1.1.max(p2.1),
            }
        } else if p1.1 == p2.1 {
            Edge::Horizontal {
                y: p1.1,
                x1: p1.0.min(p2.0),
                x2: p1.0.max(p2.0),
            }
        } else {
            unreachable!();
        }
    }
}

fn build_polygon(red_tiles: &[(i64, i64)]) -> Vec<Edge> {
    let mut edges = Vec::new();

    for i in 0..red_tiles.len() {
        let p1 = red_tiles[i];
        let p2 = red_tiles[(i + 1) % red_tiles.len()];
        edges.push(Edge::from_points(p1, p2));
    }

    edges
}

fn point_on_edge(point: (i64, i64), edge: &Edge) -> bool {
    match edge {
        Edge::Horizontal { y, x1, x2 } => point.1 == *y && point.0 >= *x1 && point.0 <= *x2,
        Edge::Vertical { x, y1, y2 } => point.0 == *x && point.1 >= *y1 && point.1 <= *y2,
    }
}

fn point_in_polygon(point: (i64, i64), edges: &[Edge]) -> bool {
    for edge in edges {
        if point_on_edge(point, edge) {
            return true;
        }
    }

    // Ray casting algorithm
    let mut count = 0;
    for edge in edges {
        if let Edge::Vertical { x, y1, y2 } = edge {
            if *x > point.0 && point.1 > *y1 && point.1 < *y2 {
                count += 1;
            }
        }
    }

    count % 2 == 1
}

fn rectangle_has_interior_intersection(
    rect: (i64, i64, i64, i64), // (x_min, x_max, y_min, y_max)
    edges: &[Edge],
) -> bool {
    let (x_min, x_max, y_min, y_max) = rect;

    for edge in edges {
        match edge {
            Edge::Horizontal { y, x1, x2 } => {
                if *y > y_min && *y < y_max {
                    let overlap_start = x1.max(&x_min);
                    let overlap_end = x2.min(&x_max);
                    if overlap_start < overlap_end {
                        return true;
                    }
                }
            }
            Edge::Vertical { x, y1, y2 } => {
                if *x > x_min && *x < x_max {
                    let overlap_start = y1.max(&y_min);
                    let overlap_end = y2.min(&y_max);
                    if overlap_start < overlap_end {
                        return true;
                    }
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day09::Day09};

    const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3\
";

    const SOLVER: Day09 = Day09;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(50));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(24));
    }
}
