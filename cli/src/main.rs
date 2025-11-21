use aoc::fmt_duration;
use clap::{Parser, Subcommand};
use std::{fs, path::Path, process::Command, time::Instant};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: CommandEnum,
}

#[derive(Subcommand, Debug)]
enum CommandEnum {
    Test {
        day: u8,
    },
    Run {
        day: u8,
        #[arg(short, long)]
        part: Option<u8>,
    },
    Generate {
        day: u8,
    },
}

fn run_part(solver: &dyn aoc::Day, input: &str, part_num: u8) {
    let start = Instant::now();
    let result = match part_num {
        1 => solver.part1(input),
        2 => solver.part2(input),
        _ => {
            eprintln!("Invalid part {part_num}");
            return;
        }
    };
    let elapsed = start.elapsed();
    println!("Part {part_num}: {result} (🚀 {})", fmt_duration(elapsed));
}

fn run_day(day: u8, part: Option<u8>) {
    let solver = match aoc::DAYS.get(&day) {
        Some(s) => s.as_ref(),
        None => {
            eprintln!("No solution found for day {day}");
            return;
        }
    };

    let input = aoc::load_input(day);

    if let Some(p) = part {
        run_part(solver, &input, p);
    } else {
        run_part(solver, &input, 1);
        run_part(solver, &input, 2);
    }
}

fn run_tests(day: u8) {
    let day_str = format!("day{:02}", day);

    let output = Command::new("cargo")
        .args(["test", &day_str, "--color=always", "-q"])
        .output()
        .expect("Failed to execute cargo test");

    print!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
        println!("Tests for {day_str} passed ✅");
    } else {
        println!("Tests for {day_str} failed ❌");
    }
}
fn generate(day: u8) {
    let day_str = format!("day{:02}", day);

    let rs_path = format!("aoc/src/days/{}.rs", day_str);
    let input_path = format!("inputs/{}.txt", day_str);

    for path in [&rs_path, &input_path] {
        if Path::new(path).exists() {
            eprintln!("File {path} already exists!");
            return;
        }
    }

    let template = format!(
        r#"use crate::Day;

pub struct Day{num:02};

impl Day for Day{num:02} {{
    fn part1(&self, input: &str) -> String {{
        String::new()
    }}

    fn part2(&self, _input: &str) -> String {{
        String::new()
    }}
}}

#[cfg(test)]
mod tests {{
    #[test]
    fn test_part1() {{}}

    #[test]
    fn test_part2() {{}}
}}
"#,
        num = day
    );

    fs::write(&rs_path, template).expect("Failed to write new day source file");
    fs::write(&input_path, "").expect("Failed to write input file");

    println!("Created:\n  {rs_path}\n  {input_path}");
}

fn main() {
    let args = Args::parse();

    match args.command {
        CommandEnum::Test { day } => run_tests(day),
        CommandEnum::Run { day, part } => run_day(day, part),
        CommandEnum::Generate { day } => generate(day),
    }
}
