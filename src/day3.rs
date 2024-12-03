use anyhow::anyhow;
use regex::Regex;

use crate::Puzzle;

pub struct Day3 {
    input: String,
}

impl Puzzle for Day3 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&mut self) -> i64 {
        let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        re.captures_iter(&self.input)
            .map(|m| m.extract::<0>().0)
            .map(|v| calc_expr(v).unwrap())
            .sum()
    }

    fn part_two(&mut self) -> i64 {
        let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
        re.captures_iter(&self.input)
            .map(|m| m.extract::<0>().0)
            .fold((true, 0), |(allowed, v), e| {
                if e.starts_with("don't") {
                    (false, v)
                } else if e.starts_with("do()") {
                    (true, v)
                } else if allowed {
                    (true, v + calc_expr(e).unwrap())
                } else {
                    (false, v)
                }
            })
            .1
    }
}

fn calc_expr(expr: &str) -> anyhow::Result<i64> {
    let num_re = Regex::new(r"\d+").unwrap();
    let operands: Vec<i64> = num_re
        .captures_iter(expr)
        .map(|v| v.extract::<0>().0.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;

    let op = expr.split('(').take(1).collect::<Vec<&str>>().join("");

    match op.as_str() {
        "mul" => Ok(operands.iter().product()),
        _ => Err(anyhow!("unknown operator")),
    }
}

#[cfg(test)]
mod tests {
    use super::{Day3, Puzzle};

    #[test]
    fn part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mut day3 = Day3::new(input.to_owned());
        assert_eq!(day3.part_one(), 161);
    }

    #[test]
    fn part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let mut day3 = Day3::new(input.to_owned());
        assert_eq!(day3.part_two(), 48);
    }
}
