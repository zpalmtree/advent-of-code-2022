extern crate regex;
extern crate num;

use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Copy,Clone)]
enum Part {
    One = 1,
    Two = 2,
}

struct Day {
    part1: fn(Vec<String>) -> String,

    part2: fn(Vec<String>) -> String,
}

static CURRENT_DAY: u32 = 9;
static CURRENT_PART: Part = Part::Two;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod tree;

fn main() {
    let days = HashMap::from([
        (1, Day{
            part1: day01::part1::solution,
            part2: day01::part2::solution
        }),
        (2, Day{
            part1: day02::part1::solution,
            part2: day02::part2::solution
        }),
        (3, Day{
            part1: day03::part1::solution,
            part2: day03::part2::solution
        }),
        (4, Day{
            part1: day04::part1::solution,
            part2: day04::part2::solution
        }),
        (5, Day{
            part1: day05::part1::solution,
            part2: day05::part2::solution
        }),
        (6, Day{
            part1: day06::part1::solution,
            part2: day06::part2::solution
        }),
        (7, Day{
            part1: day07::part1::solution,
            part2: day07::part2::solution
        }),
        (8, Day{
            part1: day08::part1::solution,
            part2: day08::part2::solution
        }),
        (9, Day{
            part1: day09::part1::solution,
            part2: day09::part2::solution
        }),
        (10, Day{
            part1: day10::part1::solution,
            part2: day10::part2::solution
        }),
        (11, Day{
            part1: day11::part1::solution,
            part2: day11::part2::solution
        }),
        (12, Day{
            part1: day12::part1::solution,
            part2: day12::part2::solution
        }),
        (13, Day{
            part1: day13::part1::solution,
            part2: day13::part2::solution
        }),
        (14, Day{
            part1: day14::part1::solution,
            part2: day14::part2::solution
        }),
        (15, Day{
            part1: day15::part1::solution,
            part2: day15::part2::solution
        }),
        (16, Day{
            part1: day16::part1::solution,
            part2: day16::part2::solution
        }),
        (17, Day{
            part1: day17::part1::solution,
            part2: day17::part2::solution
        }),
        (18, Day{
            part1: day18::part1::solution,
            part2: day18::part2::solution
        }),
        (19, Day{
            part1: day19::part1::solution,
            part2: day19::part2::solution
        }),
        (20, Day{
            part1: day20::part1::solution,
            part2: day20::part2::solution
        }),
        (21, Day{
            part1: day21::part1::solution,
            part2: day21::part2::solution
        }),
        (22, Day{
            part1: day22::part1::solution,
            part2: day22::part2::solution
        }),
        (23, Day{
            part1: day23::part1::solution,
            part2: day23::part2::solution
        }),
        (24, Day{
            part1: day24::part1::solution,
            part2: day24::part2::solution
        }),
        (25, Day{
            part1: day25::part1::solution,
            part2: day25::part2::solution
        }),
    ]);

    let input = load_input(CURRENT_DAY)
        .expect(&format!("Failed to load data for day {}!", CURRENT_DAY));

    match days.get(&CURRENT_DAY) {
        Some(Day { part1, part2 }) => run_day(input, *part1, *part2),
        _ => (),
    }
}

fn run_day(
    input: Vec<String>,
    part_one_f: fn(Vec<String>) -> String,
    part_two_f: fn(Vec<String>) -> String,
) {
    let result = match CURRENT_PART {
        Part::One => part_one_f(input),
        Part::Two => part_two_f(input),
    };

    println!("\nResult for day {}, part {}: {}", CURRENT_DAY, CURRENT_PART as u32, result);
}

fn load_input(current_day: u32) -> Result<Vec<String>> {
    let padded = if current_day < 10 {
        format!("0{}", current_day)
    } else {
        current_day.to_string()
    };

    let file = File::open(format!("./inputs/day{}.txt", padded))?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let next_line = line.expect("Expected line!");
        lines.push(next_line);
    }

    Ok(lines)
}
