use std::iter::repeat_n;

use itertools::Itertools;
use rayon::prelude::*;

use crate::Puzzle;

pub struct Day7 {
    equations: Vec<(i64, Vec<i64>)>,
}

impl Day7 {
    fn calculate(&self, valid_operators: Vec<Operators>) -> i64 {
        self.equations
            .par_iter()
            .filter(|(result, operands)| {
                repeat_n(valid_operators.iter(), operands.len())
                    .multi_cartesian_product()
                    .any(|operators| {
                        operands
                            .iter()
                            .enumerate()
                            .fold(operators[0].get_neutral(), |acc, (index, operand)| {
                                operators[index].apply(acc, *operand)
                            })
                            == *result
                    })
            })
            .map(|(result, _)| *result)
            .sum::<i64>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operators {
    Add,
    Mul,
    Concat,
}

impl Operators {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operators::Add => a + b,
            Operators::Mul => a * b,
            Operators::Concat => (a.to_string() + b.to_string().as_str())
                .parse::<i64>()
                .unwrap(),
        }
    }

    fn get_neutral(&self) -> i64 {
        match self {
            Operators::Add => 0,
            Operators::Mul => 1,
            Operators::Concat => 0,
        }
    }
}

impl Puzzle for Day7 {
    fn new(input: String) -> Self {
        let equations: Vec<(i64, Vec<i64>)> = input
            .lines()
            .map(|l| {
                let (result_str, operands_str) = l.split_once(":").unwrap();
                let operands: Vec<i64> = operands_str
                    .trim()
                    .split(" ")
                    .map(|o| o.parse::<i64>().unwrap())
                    .collect();
                (result_str.parse::<i64>().unwrap(), operands)
            })
            .collect();

        Self { equations }
    }

    fn part_one(&mut self) -> i64 {
        self.calculate(vec![Operators::Add, Operators::Mul])
    }

    fn part_two(&mut self) -> i64 {
        self.calculate(vec![Operators::Add, Operators::Mul, Operators::Concat])
    }
}

#[cfg(test)]
mod tests {
    use super::{Day7, Puzzle};

    #[test]
    fn part_one() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let mut day7 = Day7::new(input.to_string());
        assert_eq!(day7.part_one(), 3749);
    }

    #[test]
    fn part_two() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let mut day7 = Day7::new(input.to_string());
        assert_eq!(day7.part_two(), 11387);
    }
}
