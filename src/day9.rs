use itertools::Itertools;

use crate::Puzzle;

#[derive(Debug)]
pub struct Day9 {
    layout: Vec<i64>,
}

impl Puzzle for Day9 {
    fn new(input: String) -> Self {
        const RADIX: u32 = 10;

        let chars = input.trim().chars().collect::<Vec<char>>();

        let (chunks, last) = chars.as_chunks::<2>();
        let (full_raw, empty): (Vec<char>, Vec<char>) = chunks
            .iter()
            .map(|e| (e.get(0).unwrap(), e.get(1).unwrap()))
            .unzip();

        // let mut last_vec = last
        //     .to_vec()
        //     .iter()
        //     .filter(|e| e.is_ascii() && !e.is_whitespace())
        //     .map(|c| c.to_digit(RADIX).unwrap())
        //     .enumerate()
        //     .map(|(i, c)| (0..dbg!(c)).map(move |_| (i as i64)).collect())
        //     .collect();

        // full_raw.append(&mut last_vec);

        let expanded_full: Vec<Vec<i64>> = full_raw
            .iter()
            .filter(|e| e.is_ascii() && !e.is_whitespace())
            .map(|c| c.to_digit(RADIX).unwrap())
            .enumerate()
            .map(|(i, c)| (0..dbg!(c)).map(move |_| (i as i64)).collect())
            .collect();
        let len = expanded_full.len();

        let mut layout: Vec<i64> = expanded_full
            .into_iter()
            // .take(self.full.len() - 1)
            .zip(
                empty
                    .iter()
                    .filter(|e| !e.is_whitespace())
                    .map(|c| c.to_digit(RADIX).unwrap())
                    .map(|e| (0..e).map(|_| -1).collect::<Vec<i64>>()),
            )
            .flat_map(move |e| vec![e.0, e.1])
            .flatten()
            // .map(|e| e)
            .collect::<Vec<i64>>();

        let mut last = (0..last.last().unwrap().to_digit(RADIX).unwrap())
            .map(|_| len as i64)
            .collect_vec();

        layout.append(&mut last);

        Self {
            layout, // full: expanded_full,
                    // empty: empty
                    //     .iter()
                    //     .filter(|e| !e.is_whitespace())
                    //     .map(|c| c.to_digit(RADIX).unwrap())
                    //     .filter(|e| *e != 0)
                    //     .collect(),
        }
    }

    fn part_one(&mut self) -> i64 {
        while self.layout.contains(&-1) {
            let replace_index = self
                .layout
                .iter()
                .enumerate()
                .find(|(_, e)| **e == -1)
                .unwrap()
                .0;

            self.layout.swap_remove(replace_index);
        }

        // self.empty.iter().enumerate().for_each(|(i, empty)| {
        //     let block = (0..*empty)
        //         .map(|_| {
        //             if self.full.last().is_some_and(|v| v.is_empty()) {
        //                 self.full.remove(self.full.len() - 1);
        //             }
        //             self.full.last_mut().unwrap().remove(0)
        //         })
        //         .collect();

        //     let insert_index = if (i * 2) + 1 < self.full.len() {
        //         (i * 2) + 1
        //     } else {
        //         self.full.len() - 1
        //     };

        //     self.full.insert(insert_index, block);
        // });
        // .collect::<Vec<Vec<usize>>>();

        // dbg!(&self.full);
        // dbg!(&empty);

        // let mut layout = self
        //     .full
        //     .iter()
        //     // .take(self.full.len() - 1)
        //     .zip(empty.iter().take(self.full.len()))
        //     .flat_map(|e| vec![e.0, e.1])
        //     .flatten()
        //     .collect::<Vec<&usize>>();

        // let mut rest = empty
        //     .iter()
        //     .skip(self.full.len())
        //     .flatten()
        //     .rev()
        //     .collect_vec();

        // layout.append(&mut rest);

        // dbg!(&layout);

        let res: i128 = self
            .layout
            .iter()
            .enumerate()
            .map(|(i, e)| i as i128 * *e as i128)
            .sum();

        println!("{res}");

        res as i64
    }

    fn part_two(&mut self) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day9, Puzzle};

    #[test]
    fn part_one() {
        let input = "2333133121414131402";
        let mut day9 = Day9::new(input.to_owned());
        assert_eq!(day9.part_one(), 1928);
    }
}
