use crate::{input_as_num_vec, Puzzle};

pub struct Day1 {
    left_list: Vec<i64>,
    right_list: Vec<i64>,
}

impl Puzzle for Day1 {
    fn new(input: String) -> Self {
        let nums = input_as_num_vec(input);

        let pairs: Vec<(i64, i64)> = nums.iter().map(|l| (l[0], l[1])).collect();
        let (mut left_list, mut right_list): (Vec<i64>, Vec<i64>) = pairs.into_iter().unzip();
        left_list.sort();
        right_list.sort();

        Self {
            left_list,
            right_list,
        }
    }

    fn part_one(&mut self) -> i64 {
        self.left_list
            .iter()
            .zip(self.right_list.clone())
            .map(|pair| (pair.0 - pair.1).abs())
            .sum()
    }

    fn part_two(&mut self) -> i64 {
        self.left_list
            .iter()
            .map(|li| {
                li * self
                    .right_list
                    .iter()
                    .filter(|ri| *ri == li)
                    .collect::<Vec<&i64>>()
                    .len() as i64
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::{day1::Day1, Puzzle};

    #[test]
    fn part_one() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let mut day1 = Day1::new(input.to_string());
        assert_eq!(day1.part_one(), 11);
    }

    #[test]
    fn part_two() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let mut day1 = Day1::new(input.to_string());
        assert_eq!(day1.part_two(), 31);
    }
}
