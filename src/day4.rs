use crate::{input_as_num_vec, Puzzle};

pub struct Day4 {
    chars: Vec<Vec<char>>,
}

impl Puzzle for Day4 {
    fn new(input: String) -> Self {
        let chars: Vec<Vec<char>> = input
            .lines()
            .into_iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();

        Self { chars }
    }

    fn part_one(&mut self) -> i64 {
        let x_positions: Vec<(usize, usize)> = self
            .chars
            .iter()
            .enumerate()
            .flat_map(|(line_number, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, letter)| **letter == 'X')
                    .map(|(letter_num, _)| (line_number, letter_num))
            })
            .collect();
    }

    fn part_two(&mut self) -> i64 {
        0
    }
}

fn count_xmas(x_pos: (usize, usize), chars: &Vec<char>) -> i64 {}
