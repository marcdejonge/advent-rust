#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use advent_macros::parsable;
use rayon::prelude::*;

#[derive(Debug)]
#[parsable(
    map(
        separated_list1(line_ending, separated_pair(i64, space1, i64)),
        |list| {
            let (mut left, mut right): (Vec<_>, Vec<_>) = list.into_iter().unzip();
            left.sort();
            right.sort();
            ( left, right )
        }
    )
)]
struct Day {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl ExecutableDay for Day {
    type Output = i64;

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
