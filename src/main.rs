use std::fs::File;
use std::io::Read;
use std::time::Instant;

use day6::Day6;

pub(crate) trait Puzzle {
    fn new(input: String) -> Self;
    fn part_one(&mut self) -> i64;
    fn part_two(&mut self) -> i64;
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input = read_file_to_str("./inputs/day6.txt").unwrap();
    let puzzle = Day6::new(input);

    execute_puzzle(puzzle);
}

fn execute_puzzle(mut puzzle: impl Puzzle) {
    let now = Instant::now();
    println!("Part1: {}", puzzle.part_one());
    let elapsed = now.elapsed();
    println!("Part1 took {:.2?}", elapsed);

    let now = Instant::now();
    println!("Part2: {}", puzzle.part_two());
    let elapsed = now.elapsed();
    println!("Part2 took {:.2?}", elapsed);
}

pub(crate) fn read_file_to_str(path: &str) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    Ok(string)
}

pub(crate) fn input_as_num_vec(input: String) -> Vec<Vec<i64>> {
    let lines = input.lines();
    lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|l| !l.is_empty())
        .collect::<Vec<Vec<i64>>>()
}
