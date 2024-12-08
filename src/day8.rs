use std::iter::repeat_n;

use itertools::Itertools;

use crate::Puzzle;

pub struct Day8 {
    map: Vec<Vec<char>>,
    unique_letters: Vec<char>,
    width: usize,
    height: usize,
}

impl Day8 {
    fn is_in_map(&self, p: (i64, i64)) -> bool {
        let height = self.map.len() as i64;
        let width = self.map[0].len() as i64;

        p.0 >= 0 && p.0 < height && p.1 >= 0 && p.1 < width
    }
}

impl Puzzle for Day8 {
    fn new(input: String) -> Self {
        let unique_letters: Vec<char> = input
            .chars()
            .unique()
            .filter(|c| c != &'#' && c != &'.')
            .collect();

        let map: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        Self {
            width: map[0].len(),
            height: map.len(),
            map,
            unique_letters,
        }
    }

    fn part_one(&mut self) -> i64 {
        self.unique_letters
            .iter()
            .flat_map(|c| {
                find_letters(&self.map, *c)
                    .into_iter()
                    .permutations(2)
                    .flat_map(|letters| {
                        let a = letters.get(0).unwrap();
                        let b = letters.get(1).unwrap();
                        let (line_diff, char_diff) = difference(*a, *b);

                        vec![(a.0 + line_diff, a.1 + char_diff)]
                    })
                // returns the list of antenas this frequency (letter) creates
            })
            .filter(|p| self.is_in_map(*p))
            .unique()
            .count() as i64
    }

    fn part_two(&mut self) -> i64 {
        self.unique_letters
            .iter()
            .flat_map(|c| {
                find_letters(&self.map, *c)
                    .into_iter()
                    .permutations(2)
                    .flat_map(|letters| {
                        let a = letters.get(0).unwrap().clone();
                        let b = letters.get(1).unwrap();
                        let (line_diff, char_diff) = difference(a, *b);

                        (0..self.height)
                            .map(move |i| (a.0 + line_diff * i as i64, a.1 + char_diff * i as i64))
                    })
                // returns the list of antenas this frequency (letter) creates
            })
            .filter(|p| self.is_in_map(*p))
            .unique()
            .count() as i64
    }
}

fn difference(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}

fn find_letters(chars: &Vec<Vec<char>>, letter_to_find: char) -> Vec<(i64, i64)> {
    chars
        .iter()
        .enumerate()
        .flat_map(|(line_number, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, letter)| **letter == letter_to_find)
                .map(move |(letter_num, _)| (line_number as i64, letter_num as i64))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Day8, Puzzle};

    #[test]
    fn part_one() {
        let input = "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";
        let mut day8 = Day8::new(input.to_owned());
        assert_eq!(day8.part_one(), 14);
    }

    #[test]
    fn part_two() {
        let input = "##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##";

        let mut day8 = Day8::new(input.to_owned());
        assert_eq!(day8.part_two(), 34);
    }
}
