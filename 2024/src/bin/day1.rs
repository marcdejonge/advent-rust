#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use itertools::Itertools;
use rayon::prelude::*;

type NR = i64;

#[derive(Debug)]
struct Day {
    left: Vec<NR>,
    right: Vec<NR>,
}

impl ExecutableDay for Day {
    type Output = NR;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let (mut left, mut right): (Vec<NR>, Vec<NR>) = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<NR>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .unzip();
        left.sort();
        right.sort();
        Day { left, right }
    }
    fn calculate_part1(&self) -> Self::Output {
        self.left.iter().zip(self.right.iter()).map(|(l, r)| (l - r).abs()).sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let map = self.right.iter().counts_fx();
        self.left.par_iter().map(|l| *map.get(l).unwrap_or(&0) as NR * l).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example1 => 11, 31 );
    day_test!( 1 => 1889772, 23228917);
}
