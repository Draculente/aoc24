use std::{cmp::Ordering, collections::HashMap};

use crate::Puzzle;

#[derive(Debug)]
pub struct Day5 {
    rules: Vec<(i64, i64)>,
    page_updates_correct: Vec<Vec<i64>>,
    page_updates_incorrect: Vec<Vec<i64>>,
}

impl Day5 {
    fn get_shall_not_appear(rules: &Vec<(i64, i64)>, key: i64) -> Vec<i64> {
        rules
            .iter()
            .filter(|(a, _)| a == &key)
            .map(|a| a.1)
            .collect()
    }

    fn has_rule(&self, before: i64, shall_not_appear_after: i64) -> bool {
        Day5::get_shall_not_appear(&self.rules, before).contains(&shall_not_appear_after)
    }
}

impl Puzzle for Day5 {
    fn new(input: String) -> Self {
        let (raw_rules, raw_page_updates) = input.split_once("\n\n").unwrap();
        let rules: Vec<(i64, i64)> = raw_rules
            .lines()
            .filter_map(|l| l.split_once("|"))
            .map(|(before, after)| {
                (
                    after.parse::<i64>().unwrap(),
                    before.parse::<i64>().unwrap(),
                )
            })
            .collect();

        let (page_updates_correct, page_updates_incorrect): (Vec<Vec<i64>>, Vec<Vec<i64>>) =
            raw_page_updates
                .lines()
                .map(|l| {
                    l.split(",")
                        .map(|v| v.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .partition(|line| {
                    line.iter().enumerate().all(|(i, page)| {
                        let shall_not_appear: Vec<i64> = Day5::get_shall_not_appear(&rules, *page);
                        line.iter()
                            .skip(i + 1)
                            .all(|x| shall_not_appear.iter().find(|s| *s == x).is_none())
                    })
                });

        Self {
            rules,
            page_updates_correct,
            page_updates_incorrect,
        }
    }

    fn part_one(&mut self) -> i64 {
        self.page_updates_correct
            .iter()
            .filter_map(|line| line.get(line.len() / 2_usize))
            .sum()
    }

    fn part_two(&mut self) -> i64 {
        let mut page_updates: Vec<Vec<i64>> = self.page_updates_incorrect.clone();
        page_updates
            .iter_mut()
            .map(|line| {
                line.sort_by(|a, b| {
                    if self.has_rule(*a, *b) {
                        Ordering::Greater
                    } else if self.has_rule(*b, *a) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });
                line
            })
            .filter_map(|line| line.get(line.len() / 2_usize))
            .sum()
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

    #[test]
    fn part_two() {
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
        assert_eq!(day1.part_two(), 123);
    }
}
