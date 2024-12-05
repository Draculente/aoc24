use std::collections::HashMap;

use crate::Puzzle;

pub struct Day5 {
    rules: HashMap<i64, i64>,
    page_updates: Vec<Vec<i64>>,
}

impl Puzzle for Day5 {
    fn new(input: String) -> Self {
        let (raw_rules, raw_page_updates) = input.split_once("\n\n").unwrap();
        let rules: HashMap<i64, i64> = raw_rules
            .lines()
            .into_iter()
            .filter_map(|l| l.split_once("|"))
            .map(|(before, after)| {
                (
                    after.parse::<i64>().unwrap(),
                    before.parse::<i64>().unwrap(),
                )
            })
            .collect();

        let page_updates: Vec<Vec<i64>> = raw_page_updates
            .lines()
            .map(|l| {
                l.split(",")
                    .into_iter()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();

        Self {
            rules,
            page_updates,
        }
    }

    fn part_one(&mut self) -> i64 {
        self.page_updates
            .iter()
            .filter(|line| {
                line.iter().enumerate().all(|(i, page)| {
                    let shall_not_appear = self.rules.get(page);
                    line.iter()
                        .position(|x| shall_not_appear.filter(|s| s == &x).is_some())
                        .filter(|p| p <= &i)
                        .is_some()
                })
            })
            .filter_map(|line| line.get(line.len() / 2 as usize))
            .sum()
    }

    fn part_two(&mut self) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day5::Day5, Puzzle};

    #[test]
    fn part_one() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let mut day1 = Day5::new(input.to_string());
        assert_eq!(day1.part_one(), 143);
    }
}
