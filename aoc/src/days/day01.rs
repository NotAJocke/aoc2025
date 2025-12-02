use crate::{Day, Solution};

pub struct Day01;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    pub fn parse(input: &str) -> Rotation {
        let direction = input.chars().next().unwrap();
        let amount = input[1..].parse().unwrap();

        match direction {
            'L' => Rotation::Left(amount),
            'R' => Rotation::Right(amount),
            _ => unreachable!(),
        }
    }
}

struct Dial<C: FnMut(i32) = fn(i32)> {
    value: i32,
    callback: Option<C>,
}

impl<C: FnMut(i32)> Dial<C> {
    pub fn new(initial: i32) -> Self {
        Self {
            value: initial,
            callback: None,
        }
    }

    pub fn on_click(&mut self, cb: C) {
        self.callback = Some(cb);
    }

    pub fn apply(&mut self, rotation: Rotation) -> i32 {
        let step = match rotation {
            Rotation::Left(_) => -1,
            Rotation::Right(_) => 1,
        };

        let count = match rotation {
            Rotation::Left(n) | Rotation::Right(n) => n,
        };

        for _ in 0..count {
            self.value = (self.value + step) % 100;
            if let Some(cb) = &mut self.callback {
                cb(self.value);
            }
        }

        self.value
    }
}

impl Day for Day01 {
    fn part1(&self, input: &str) -> Solution {
        let mut dial = Dial::<fn(i32)>::new(50);

        Solution::Int(
            input
                .trim()
                .lines()
                .map(Rotation::parse)
                .filter(|&r| dial.apply(r) == 0)
                .count() as i64,
        )
    }

    fn part2(&self, input: &str) -> Solution {
        let mut dial = Dial::new(50);

        let mut zeroes = 0;
        dial.on_click(|x| {
            if x == 0 {
                zeroes += 1;
            }
        });

        input
            .trim()
            .lines()
            .map(Rotation::parse)
            .for_each(|rotation| {
                dial.apply(rotation);
            });

        Solution::Int(zeroes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day01::Day01};

    const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const SOLVER: Day01 = Day01;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(3));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(6));
    }
}
