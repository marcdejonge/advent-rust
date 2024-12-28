#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use fxhash::FxHashMap;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Parser;
use rayon::prelude::*;

struct Day {
    secrets: Vec<i32>,
}

#[inline]
fn mix_and_prune(value: i32, secret: i32) -> i32 { (value ^ secret) & 0xffffff }

fn secrets(secret: i32) -> impl Iterator<Item = i32> {
    std::iter::successors(Some(secret), |&n| {
        let n = mix_and_prune(n << 6, n);
        let n = mix_and_prune(n >> 5, n);
        Some(mix_and_prune(n << 11, n))
    })
}

fn price_changes(secret: i32) -> impl Iterator<Item = (i32, i32)> {
    secrets(secret)
        .zip_with_next()
        .map(|(prev, next)| (next % 10, (next % 10) - (prev % 10)))
}

type Prices = FxHashMap<[i32; 4], i32>;

fn combine_prices(left: Prices, right: Prices) -> Prices {
    left.into_iter().fold(right, |mut acc, (k, v)| {
        acc.entry(k).and_modify(|e| *e += v).or_insert(v);
        acc
    })
}

fn find_possible_price_changes(secret: i32) -> Prices {
    let mut possible_changes = Prices::default();
    price_changes(secret).take(2001).windowed().for_each(|changes| {
        possible_changes
            .entry(changes.map(|(_, change)| change))
            .or_insert(changes[3].0);
    });
    possible_changes
}

impl ExecutableDay for Day {
    type Output = u64;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(separated_list1(line_ending, complete::i32), |secrets| Day {
            secrets,
        })
    }
    fn calculate_part1(&self) -> Self::Output {
        self.secrets.iter().map(|&s| secrets(s).take(2001).last().unwrap() as u64).sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        self.secrets
            .par_iter()
            .map(|&s| find_possible_price_changes(s))
            .reduce(FxHashMap::default, combine_prices)
            .values()
            .copied()
            .max()
            .unwrap() as u64
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 22, example1 => 37327623, 24 );
    day_test!( 22, example2 => 37990510, 23 );
    day_test!( 22 => 16619522798, 1854 );
}