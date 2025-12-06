use crate::{Day, Solution};

pub struct Day06;

impl Day for Day06 {
    fn part1(&self, input: &str) -> Solution {
        let (groups, operations) = parse_input(input.trim());

        Solution::Int(apply_operations(groups, operations))
    }

    fn part2(&self, input: &str) -> Solution {
        let (groups, operations) = parse_input2(input.trim());

        Solution::Int(apply_operations(groups, operations))
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Mul,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Mul => a * b,
            Operation::Plus => a + b,
        }
    }

    fn parse(word: &str) -> Operation {
        match word {
            "*" => Operation::Mul,
            "+" => Operation::Plus,
            _ => unreachable!(),
        }
    }

    fn identity(&self) -> i64 {
        match self {
            Operation::Plus => 0,
            Operation::Mul => 1,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<Operation>) {
    let mut cols: Option<Vec<Vec<i64>>> = None;
    let mut ops = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let first_char = line.chars().next().unwrap();

        if first_char.is_ascii_digit() {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            if cols.is_none() {
                cols = Some(vec![Vec::new(); numbers.len()]);
            }

            for (i, &num) in numbers.iter().enumerate() {
                if let Some(ref mut c) = cols {
                    c[i].push(num);
                }
            }
        } else {
            ops = line.split_whitespace().map(Operation::parse).collect();
        }
    }

    (cols.unwrap(), ops)
}

fn apply_operations(groups: Vec<Vec<i64>>, operations: Vec<Operation>) -> i64 {
    groups
        .into_iter()
        .zip(operations)
        .map(|(numbers, op)| {
            numbers
                .into_iter()
                .fold(op.identity(), |acc, x| op.apply(acc, x))
        })
        .sum()
}

fn transpose(input: &str) -> String {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    if lines.is_empty() {
        return String::new();
    }

    let rows = lines.len();
    let cols = lines.iter().map(|line| line.len()).max().unwrap();

    let mut result = Vec::with_capacity(rows * cols + rows);

    for col in 0..cols {
        for line in &lines {
            if col < line.len() {
                result.push(line[col]);
            } else {
                result.push(b' ');
            }
        }
        result.push(b'\n');
    }

    // Safety: all bytes are valid ASCII, so this is safe
    unsafe { String::from_utf8_unchecked(result) }
}

fn parse_input2(input: &str) -> (Vec<Vec<i64>>, Vec<Operation>) {
    let (grid_str, ops_str) = input.rsplit_once("\n").unwrap();

    let ops = ops_str
        .split_whitespace()
        .map(Operation::parse)
        .collect::<Vec<_>>();

    let transposed = transpose(grid_str);

    let mut groups: Vec<Vec<i64>> = Vec::new();
    let mut current: Vec<i64> = Vec::new();

    for line in transposed.lines().map(str::trim) {
        if line.is_empty() {
            groups.push(current);
            current = Vec::new();
        } else {
            let number = line.parse().unwrap();
            current.push(number);
        }
    }
    groups.push(current);

    (groups, ops)
}

#[cfg(test)]
mod tests {
    use crate::{Day, Solution, days::day06::Day06};

    const TEST: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    const SOLVER: Day06 = Day06;

    #[test]
    fn test_part1() {
        assert_eq!(SOLVER.part1(TEST), Solution::Int(4277556));
    }

    #[test]
    fn test_part2() {
        assert_eq!(SOLVER.part2(TEST), Solution::Int(3263827));
    }
}

// rocket science, i'll let this here so i can remember
// that i should never make assumptions based on test input
// fn transpose_digits(numbers: Vec<i64>, op: Operation) -> Vec<i64> {
//     let mut max_len = 0;
//     for &n in &numbers {
//         let len = num_len(n);

//         if len > max_len {
//             max_len = len;
//         }
//     }

//     let mut transposed = vec![0; max_len as usize];

//     match op {
//         Operation::Mul => {
//             for digit_pos in 0..max_len {
//                 let mut new_number = 0;
//                 for &num in &numbers {
//                     if num_len(num) <= digit_pos {
//                         continue;
//                     }

//                     let digit = (num / 10_i64.pow(digit_pos)) % 10;

//                     new_number = new_number * 10 + digit;
//                 }

//                 transposed[digit_pos as usize] = new_number;
//             }
//         }
//         Operation::Plus => {
//             for digit_pos in 0..max_len {
//                 let mut new_number = 0;
//                 for &num in &numbers {
//                     let len = num_len(num);
//                     if len <= digit_pos {
//                         continue;
//                     }

//                     let digit = (num / 10_i64.pow(len - digit_pos - 1)) % 10;

//                     new_number = new_number * 10 + digit;
//                 }

//                 transposed[digit_pos as usize] = new_number;
//             }
//         }
//     };

//     transposed
// }

// fn num_len(num: i64) -> u32 {
//     if num == 0 {
//         1
//     } else {
//         num.checked_ilog10().unwrap() + 1
//     }
// }
