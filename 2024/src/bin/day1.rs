#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use rayon::prelude::*;

#[derive(Debug)]
struct Day {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl ExecutableDay for Day {
    type Output = i64;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                line_ending,
                separated_pair(complete::i64, space1, complete::i64),
            ),
            |list| {
                let (mut left, mut right): (Vec<_>, Vec<_>) = list.into_iter().unzip();
                left.sort();
                right.sort();
                Day { left, right }
            },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        self.left.iter().zip(self.right.iter()).map(|(l, r)| (l - r).abs()).sum()
    }
    fn calculate_part2(&self) -> Self::AltOutput {
        let map = self.right.iter().counts_fx();
        self.left.par_iter().map(|l| *map.get(l).unwrap_or(&0) as i64 * l).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example1 => 11, 31 );
    day_test!( 1 => 1889772, 23228917);
}
