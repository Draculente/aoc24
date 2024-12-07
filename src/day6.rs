use std::collections::HashSet;

use itertools::Itertools;

use crate::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_position(&self, position: (usize, usize)) -> (i64, i64) {
        let step: (i64, i64) = match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        (position.0 as i64 + step.0, position.1 as i64 + step.1)
    }

    fn from_char(c: &char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("Not a valid direction char"),
        }
    }
}

pub struct Day6 {
    map: Vec<Vec<char>>,
    guard: Guard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    direction: Direction,
    // (line_from_top, char_from_left)
    position: (usize, usize),
}

impl Guard {
    fn turn_90(self) -> Self {
        let direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        Self {
            direction,
            position: self.position,
        }
    }

    fn advance(self, new_position: (usize, usize)) -> Self {
        Self {
            direction: self.direction,
            position: new_position,
        }
    }
}

impl Day6 {
    fn step(&self, guard: Guard, extra_block_pos: Option<(usize, usize)>) -> Option<Guard> {
        if let Some(in_front) = self.is_in_map(guard.direction.next_position(guard.position)) {
            let in_front_char = self.get_char(in_front).unwrap();

            if in_front_char == '#' || extra_block_pos.is_some_and(|pos| pos == in_front) {
                return Some(guard.turn_90());
            } else {
                return Some(guard.advance(in_front));
            }
        };
        None
    }

    fn get_height(&self) -> usize {
        self.map.len()
    }

    fn get_width(&self) -> usize {
        self.map[0].len()
    }

    fn is_in_map(&self, position: (i64, i64)) -> Option<(usize, usize)> {
        let height = self.get_height() as i64;
        let width = self.get_width() as i64;

        let (h, w) = position;

        if h < 0 || h >= height {
            return None;
        }
        if w < 0 || w >= width {
            return None;
        }

        Some((h as usize, w as usize))
    }

    fn get_char(&self, position: (usize, usize)) -> Option<char> {
        self.map
            .get(position.0)
            .and_then(|l| l.get(position.1))
            .cloned()
    }

    fn has_loop(&self, extra_block_pos: Option<(usize, usize)>) -> bool {
        let mut positions = HashSet::new();
        let mut guard = self.guard;
        while let Some(new_guard) = self.step(guard, extra_block_pos) {
            if !positions.insert(new_guard) {
                return true;
            }
            guard = new_guard;
        }
        false
    }
}

impl Puzzle for Day6 {
    fn new(input: String) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let binding = map
            .iter()
            .enumerate()
            .filter_map(|(line_index, line)| {
                line.iter()
                    .enumerate()
                    .find(|(_, c)| ['^', '>', '<', 'v'].contains(c))
                    .map(|(char_index, c)| (Direction::from_char(c), (line_index, char_index)))
            })
            .collect::<Vec<(Direction, (usize, usize))>>();

        let (direction, position) = binding.get(0).unwrap();
        Self {
            map,
            guard: Guard {
                position: position.to_owned(),
                direction: direction.to_owned(),
            },
        }
    }

    fn part_one(&mut self) -> i64 {
        let mut positions = vec![];
        let mut guard = self.guard;
        while let Some(new_guard) = self.step(guard, None) {
            positions.push(new_guard.position);
            guard = new_guard;
        }
        positions.iter().unique().count() as i64
    }

    fn part_two(&mut self) -> i64 {
        let width = self.get_width();
        let height = self.get_height();

        (0..width)
            .into_iter()
            .flat_map(|w| (0..height).into_iter().map(move |h| (h, w)))
            .filter(|pos| *pos != self.guard.position)
            .filter(|block_pos| self.has_loop(Some(*block_pos)))
            .count() as i64
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Day6, Direction, Puzzle};

    #[test]
    fn part_one() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut day6 = Day6::new(input.to_string());
        assert_eq!(day6.part_one(), 41);
    }

    #[test]
    fn part_two() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut day6 = Day6::new(input.to_string());
        assert_eq!(day6.part_two(), 6);
    }

    #[test]
    fn test_hashset() {
        let mut set = HashSet::new();
        set.insert((Direction::Up, (1, 2)));
        assert_eq!(set.insert((Direction::Up, (1, 2))), false);
    }
}
