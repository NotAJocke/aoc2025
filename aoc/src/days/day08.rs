use rustc_hash::FxHashMap;

use crate::{Day, Solution, dsu::Dsu};

pub struct Day08;

impl Day for Day08 {
    fn part1(&self, input: &str) -> Solution {
        let points = parse_input(input);
        let n = points.len();

        let mut edges = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = euclidian_distance(points[i], points[j]);
                edges.push((dist, i, j));
            }
        }

        edges.sort_unstable_by_key(|e| e.0);

        let connection_limit = if n == 20 { 10 } else { 1000 };

        let mut dsu = Dsu::new(n);

        for i in 0..connection_limit {
            if i >= edges.len() {
                break;
            }
            let (_, u, v) = edges[i];
            dsu.union(u, v);
        }

        let mut component_sizes: FxHashMap<usize, i64> = FxHashMap::default();
        for i in 0..n {
            let root = dsu.find(i);
            *component_sizes.entry(root).or_default() += 1;
        }

        let mut sizes: Vec<i64> = component_sizes.values().cloned().collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        let result: i64 = sizes.iter().take(3).product();

        Solution::Int(result)
    }

    fn part2(&self, input: &str) -> Solution {
        let points = parse_input(input);
        let n = points.len();

        let edges = get_sorted_edges(&points);

        let mut dsu = Dsu::new(n);
        let mut components = n;

        for (_, u, v) in edges {
            if dsu.find(u) != dsu.find(v) {
                dsu.union(u, v);
                components -= 1;

                if components == 1 {
                    let ans = points[u].0 * points[v].0;
                    return Solution::Int(ans);
                }
            }
        }

        Solution::Int(0)
    }
}

type Coord = (i64, i64, i64);

fn euclidian_distance(a: Coord, b: Coord) -> i64 {
    let x = (a.0 - b.0).pow(2);
    let y = (a.1 - b.1).pow(2);
    let z = (a.2 - b.2).pow(2);

    x + y + z
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )
        })
        .collect()
}

fn get_sorted_edges(points: &[Coord]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = euclidian_distance(points[i], points[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    edges
}

#[cfg(test)]
mod tests {
    use crate::{
        Day, Solution,
        days::day08::{Day08, euclidian_distance},
    };

    const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    const SOLVER: Day08 = Day08;

    #[test]
    fn euclidian() {
        let a = (1, 2, 0);
        let b = (4, 6, 12);

        assert_eq!(euclidian_distance(a, b), 169);
    }

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(40));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(25272));
    }
}
