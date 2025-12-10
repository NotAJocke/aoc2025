use std::collections::VecDeque;

use crate::{Day, Solution};
use good_lp::{Expression, Solution as _, SolverModel, default_solver, variable, variables};

pub struct Day10;

impl Day for Day10 {
    fn part1(&self, input: &str) -> Solution {
        let mut total_presses = 0;

        for line in input.lines() {
            let machine = Machine::from(line);

            let min_presses =
                bfs_min_presses(machine.lights, machine.target_lights, &machine.buttons);
            total_presses += min_presses as i64;
        }

        Solution::Int(total_presses)
    }

    fn part2(&self, input: &str) -> Solution {
        let mut total_presses = 0;

        for line in input.lines() {
            let machine = Machine::from(line);

            let presses = solve_machine_lp(&machine);
            total_presses += presses;
        }

        Solution::Int(total_presses)
    }
}

fn solve_machine_lp(machine: &Machine) -> i64 {
    let mut problem = variables!();

    let vars: Vec<_> = (0..machine.buttons.len())
        .map(|_| problem.add(variable().min(0).integer()))
        .collect();

    let objective: Expression = vars.iter().sum();
    let mut model = problem.minimise(objective).using(default_solver);

    for (counter_idx, &target_val) in machine.joltages.iter().enumerate() {
        let mut expr = Expression::from(0);

        for (btn_idx, &btn_mask) in machine.buttons.iter().enumerate() {
            if (btn_mask >> counter_idx) & 1 == 1 {
                expr += vars[btn_idx];
            }
        }

        model.add_constraint(expr.eq(target_val as f64));
    }

    match model.solve() {
        Ok(solution) => solution.eval(vars.iter().sum::<Expression>()).round() as i64,
        Err(_) => 0,
    }
}

fn bfs_min_presses(start: u16, target: u16, buttons: &[u16]) -> u32 {
    if start == target {
        return 0;
    }

    let max_bit = buttons
        .iter()
        .chain(&[start, target])
        .map(|&x| 16 - x.leading_zeros())
        .max()
        .unwrap_or(0) as usize;

    let state_count = 1 << max_bit;
    let mut visited = vec![false; state_count];
    let mut queue = VecDeque::new();

    visited[start as usize] = true;
    queue.push_back((start, 0));

    while let Some((current_state, distance)) = queue.pop_front() {
        for &button_mask in buttons {
            let next_state = current_state ^ button_mask;

            if next_state == target {
                return distance + 1;
            }

            if !visited[next_state as usize] {
                visited[next_state as usize] = true;
                queue.push_back((next_state, distance + 1));
            }
        }
    }

    u32::MAX
}

fn parse_lights(s: &str) -> u16 {
    let mut target_lights: u16 = 0;

    for (i, c) in s.chars().filter(|c| matches!(c, '.' | '#')).enumerate() {
        if c == '#' {
            target_lights |= 1 << i;
        }
    }

    target_lights
}

fn parse_button(buttons: &mut Vec<u16>, s: &str) {
    let inner = &s[1..s.len() - 1];

    let mut mask: u16 = 0;
    let mut num: usize = 0;
    let mut reading_num = false;

    for ch in inner.bytes() {
        match ch {
            b'0'..=b'9' => {
                reading_num = true;
                num = num * 10 + (ch - b'0') as usize;
            }
            b',' => {
                if reading_num {
                    mask |= 1 << num;
                    num = 0;
                    reading_num = false;
                }
            }
            _ => {}
        }
    }
    if reading_num {
        mask |= 1 << num;
    }

    buttons.push(mask);
}

fn parse_joltages(joltages: &mut Vec<i64>, s: &str) {
    let inner = &s[1..s.len() - 1];

    let mut num: i64 = 0;
    let mut reading_num = false;

    for ch in inner.bytes() {
        match ch {
            b'0'..=b'9' => {
                reading_num = true;
                num = num * 10 + (ch - b'0') as i64;
            }
            b',' => {
                if reading_num {
                    joltages.push(num);

                    num = 0;
                    reading_num = false;
                }
            }
            _ => {}
        }
    }

    if reading_num {
        joltages.push(num);
    }
}

#[derive(Debug)]
struct Machine {
    lights: u16,
    target_lights: u16,
    buttons: Vec<u16>,
    #[allow(dead_code)]
    joltages: Vec<i64>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.trim().split_whitespace();

        let target_lights = parse_lights(parts.next().unwrap());

        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        for part in parts {
            if part.starts_with('(') && part.ends_with(')') {
                parse_button(&mut buttons, part);
            } else if part.starts_with('{') && part.ends_with('}') {
                parse_joltages(&mut joltages, part);
            }
        }

        Self {
            lights: 0,
            target_lights,
            buttons,
            joltages,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day10::Day10};

    const TEST: &str = "\
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\
    ";

    const SOLVER: Day10 = Day10;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(7));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(33))
    }
}
