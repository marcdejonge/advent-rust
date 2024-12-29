#![feature(test)]

use advent_lib::day_main;
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
struct Numbers {
    left: Vec<i64>,
    right: Vec<i64>,
}

fn calculate_part1(numbers: &Numbers) -> i64 {
    numbers.left.iter().zip(numbers.right.iter()).map(|(l, r)| (l - r).abs()).sum()
}

fn calculate_part2(numbers: &Numbers) -> i64 {
    let map = numbers.right.iter().counts_fx();
    numbers.left.par_iter().map(|l| *map.get(l).unwrap_or(&0) as i64 * l).sum()
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example1 => 11, 31 );
    day_test!( 1 => 1889772, 23228917);
}
