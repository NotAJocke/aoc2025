use rustc_hash::FxHashMap;

use crate::{Day, Solution};

pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: &str) -> Solution {
        let start = "you";
        let end = "out";

        let graph = Graph::from(input.trim());

        Solution::Int(graph.n_path_from_to(start, end))
    }

    fn part2(&self, input: &str) -> Solution {
        let start = "svr";
        let end = "out";

        let graph = Graph::from(input.trim());

        Solution::Int(graph.n_path_from_to2(start, end))
    }
}

#[derive(Debug)]
struct Graph<'a> {
    adjency: FxHashMap<&'a str, Vec<&'a str>>,
}

impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        let adjency = value
            .lines()
            .map(|line| {
                let (node_name, rest) = line.split_once(':').unwrap();

                (node_name, rest.split_whitespace().collect())
            })
            .collect();

        Graph { adjency }
    }
}

impl<'a> Graph<'a> {
    pub fn n_path_from_to(&self, node: &str, to: &str) -> i64 {
        let mut memo = FxHashMap::default();

        return self.n_path_from_to_helper(node, to, &mut memo);
    }

    fn n_path_from_to_helper(
        &'a self,
        node: &'a str,
        to: &'a str,
        memo: &mut FxHashMap<&'a str, i64>,
    ) -> i64 {
        if node == to {
            return 1;
        }

        if let Some(&val) = memo.get(node) {
            return val;
        }

        let childs = self.adjency.get(node).unwrap();
        let mut total = 0;

        for child in childs {
            total += self.n_path_from_to_helper(child, to, memo);
        }

        memo.insert(node, total);

        total
    }

    // PART 2 \\

    pub fn n_path_from_to2(&self, node: &str, to: &str) -> i64 {
        let mut memo = FxHashMap::default();

        return self.n_path_from_to_helper2(node, to, &mut memo, false, false);
    }

    fn n_path_from_to_helper2(
        &'a self,
        node: &'a str,
        to: &'a str,
        memo: &mut FxHashMap<(&'a str, bool, bool), i64>,
        dac: bool,
        fft: bool,
    ) -> i64 {
        if node == to {
            if dac && fft {
                return 1;
            } else {
                return 0;
            }
        }

        if let Some(&val) = memo.get(&(node, dac, fft)) {
            return val;
        }

        let dac = dac || node == "dac";
        let fft = fft || node == "fft";

        let childs = self.adjency.get(node).unwrap();
        let mut total = 0;

        for child in childs {
            total += self.n_path_from_to_helper2(child, to, memo, dac, fft);
        }

        memo.insert((node, dac, fft), total);

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day11::Day11};

    const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out\
";

    const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out\
";

    const SOLVER: Day11 = Day11;

    #[test]
    fn test_part1() {
        let result = SOLVER.part1(TEST);
        assert_eq!(result, Solution::Int(5));
    }

    #[test]
    fn test_part2() {
        let result = SOLVER.part2(TEST2);
        assert_eq!(result, Solution::Int(2));
    }
}
