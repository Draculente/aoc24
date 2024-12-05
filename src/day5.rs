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
        // self.page_updates.iter().all(|line| line.iter().enumerate().all(|(i, page)| line.iter().position(|x| )))
    }

    fn part_two(&mut self) -> i64 {
        todo!()
    }
}
