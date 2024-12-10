use std::collections::VecDeque;

use itertools::Itertools;

use crate::{input_as_num_vec_digts, Puzzle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_diff(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

pub struct Day10 {
    map: Vec<Vec<i64>>,
}

impl Day10 {
    fn get_reachables(&self, pos: (i64, i64)) -> Vec<(i64, i64)> {
        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        directions
            .iter()
            .map(|e| e.to_diff())
            .map(|e| (pos.0 + e.0, pos.1 + e.1))
            .filter_map(|e| self.is_in_map(e))
            .filter(|e| self.get(*e) - self.get(pos) == 1)
            .collect()
    }

    fn is_in_map(&self, position: (i64, i64)) -> Option<(i64, i64)> {
        let height = self.map.len() as i64;
        let width = self.map[0].len() as i64;

        let (h, w) = position;

        if h < 0 || h >= height {
            return None;
        }
        if w < 0 || w >= width {
            return None;
        }

        Some((h as i64, w as i64))
    }

    fn get(&self, pos: (i64, i64)) -> i64 {
        self.map
            .get(pos.0 as usize)
            .and_then(|e| e.get(pos.1 as usize))
            .cloned()
            .unwrap()
    }
}

impl Puzzle for Day10 {
    fn new(input: String) -> Self {
        Self {
            map: input_as_num_vec_digts(input),
        }
    }

    fn part_one(&mut self) -> i64 {
        find_entries(&self.map, 0)
            .iter()
            .map(|start_pos| get_score(&self, *start_pos))
            .sum()
    }

    fn part_two(&mut self) -> i64 {
        find_entries(&self.map, 0)
            .iter()
            .map(|start_pos| get_rating(&self, *start_pos))
            .sum()
    }
}

fn get_rating(map: &Day10, start_pos: (i64, i64)) -> i64 {
    let mut queue = VecDeque::from(vec![start_pos]);

    let mut nine_count = 0;

    while let Some(f) = queue.pop_front() {
        let reachable = map.get_reachables(f);
        let (nines, none_nines): (Vec<(i64, i64)>, Vec<(i64, i64)>) =
            reachable.iter().partition(|pos| map.get(**pos) == 9);
        nine_count += nines.len();
        none_nines.iter().for_each(|e| queue.push_back(*e));
    }

    nine_count as i64
}

fn get_score(map: &Day10, start_pos: (i64, i64)) -> i64 {
    let mut queue = VecDeque::from(vec![start_pos]);

    let mut nine_pos = vec![];

    while let Some(f) = queue.pop_front() {
        let reachable = map.get_reachables(f);
        let (mut nines, none_nines): (Vec<(i64, i64)>, Vec<(i64, i64)>) =
            reachable.iter().partition(|pos| map.get(**pos) == 9);
        nine_pos.append(&mut nines);
        none_nines.iter().for_each(|e| {
            if !queue.contains(e) {
                queue.push_back(*e)
            }
        });
    }

    nine_pos.iter().unique().count() as i64
}

fn find_entries(map: &Vec<Vec<i64>>, num_to_find: i64) -> Vec<(i64, i64)> {
    map.iter()
        .enumerate()
        .flat_map(|(line_number, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, letter)| **letter == num_to_find)
                .map(move |(letter_num, _)| (line_number as i64, letter_num as i64))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;

    use super::Day10;

    #[test]
    fn part_one() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let mut day10 = Day10::new(input.to_string());
        assert_eq!(day10.part_one(), 36);
    }

    #[test]
    fn part_two() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let mut day10 = Day10::new(input.to_string());
        assert_eq!(day10.part_two(), 81);
    }
}
