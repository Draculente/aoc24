use crate::Puzzle;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

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
        find_letters(&self.chars, 'X')
            .iter()
            .map(|e| count_xmas(e.to_owned(), &self.chars))
            .sum()
    }

    fn part_two(&mut self) -> i64 {
        find_letters(&self.chars, 'A')
            .iter()
            .filter(|e| check_for_x(e.to_owned().to_owned(), &self.chars))
            .count() as i64
    }
}

fn check_for_x((a_pos_line, a_pos_char): (i64, i64), chars: &Vec<Vec<char>>) -> bool {
    let positions = vec![
        (a_pos_line + 1, a_pos_char - 1),
        (a_pos_line - 1, a_pos_char - 1),
        (a_pos_line - 1, a_pos_char + 1),
        (a_pos_line + 1, a_pos_char + 1),
    ];

    let vals: Vec<&char> = positions
        .iter()
        .filter(|(a, b)| a >= &0 && b >= &0)
        .filter_map(|(line, c)| chars.get(*line as usize).and_then(|l| l.get(*c as usize)))
        .collect();

    vals.len() == 4
        && vals.char_count(&&'M') == vals.char_count(&&'S')
        && vals.windows(2).any(|w| w[0] == w[1])
}

trait CharCounter<A> {
    fn char_count(self, c: A) -> usize;
}

impl<'a, A: PartialEq> CharCounter<A> for &'a Vec<A> {
    fn char_count(self, c: A) -> usize {
        self.iter().filter(|l| l == &&c).count()
    }
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

fn count_xmas((x_pos_line, x_pos_char): (i64, i64), chars: &Vec<Vec<char>>) -> i64 {
    let directions: Vec<Box<dyn Fn(i64) -> (i64, i64)>> = vec![
        // forward
        Box::new(|i| (x_pos_line, x_pos_char + i)),
        // backward
        Box::new(|i| (x_pos_line, x_pos_char - i)),
        // up
        Box::new(|i| (x_pos_line + i, x_pos_char)),
        // down
        Box::new(|i| (x_pos_line - i, x_pos_char)),
        // top_left
        Box::new(|i| (x_pos_line - i, x_pos_char - i)),
        // top_right
        Box::new(|i| (x_pos_line - i, x_pos_char + i)),
        // bottom_left
        Box::new(|i| (x_pos_line + i, x_pos_char - i)),
        // bottom_right
        Box::new(|i| (x_pos_line + i, x_pos_char + i)),
    ];

    directions.iter().filter(|f| check_xmas(f, chars)).count() as i64
}

fn check_xmas(f: impl Fn(i64) -> (i64, i64), chars: &Vec<Vec<char>>) -> bool {
    (0..XMAS.len()).into_iter().all(|i| {
        let (line, char) = f(i as i64);
        !(line < 0 || char < 0)
            && chars.get(line as usize).and_then(|c| c.get(char as usize)) == XMAS.get(i)
    })
}

#[cfg(test)]
mod tests {
    use crate::{day4::Day4, Puzzle};

    #[test]
    fn part_one() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let mut day1 = Day4::new(input.to_string());
        assert_eq!(day1.part_one(), 18);
    }

    #[test]
    fn part_two() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let mut day1 = Day4::new(input.to_string());
        assert_eq!(day1.part_two(), 9);
    }

    #[test]
    fn part_two_jumble() {
        let input = "M.S
.A.
S.M";
        let mut day1 = Day4::new(input.to_string());
        assert_eq!(day1.part_two(), 0);
    }
}

/*

*/
