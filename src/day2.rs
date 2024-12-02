use crate::{input_as_num_vec, Puzzle};

pub struct Day2 {
    lines: Vec<Vec<i64>>,
}

impl Puzzle for Day2 {
    fn new(input: String) -> Self {
        Self {
            lines: input_as_num_vec(input),
        }
    }

    fn part_one(&mut self) -> i64 {
        self.lines.iter().filter(|l| report_is_safe(l)).count() as i64
    }

    fn part_two(&mut self) -> i64 {
        self.lines
            .iter()
            .filter(|l| {
                (0..l.len())
                    .map(|i| {
                        let mut l: Vec<i64> = l.to_vec();
                        l.remove(i);
                        l
                    })
                    .any(|l| report_is_safe(&l))
            })
            .count() as i64
    }
}

fn report_is_safe(report: &Vec<i64>) -> bool {
    report
        .windows(2)
        .all(|w| (w[0] - w[1]) < 0 && (w[0] - w[1]) > -4)
        || report
            .windows(2)
            .all(|w| (w[0] - w[1]) > 0 && (w[0] - w[1]) < 4)
}

#[cfg(test)]
mod tests {
    use super::{Day2, Puzzle};

    #[test]
    fn part_one() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let mut day2 = Day2::new(input.to_owned());
        assert_eq!(day2.part_one(), 2);
    }

    #[test]

    fn part_two() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let mut day2 = Day2::new(input.to_owned());
        assert_eq!(day2.part_two(), 4);
    }
}
