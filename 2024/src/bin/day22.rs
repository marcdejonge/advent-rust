#![feature(test)]

use advent_lib::iter_utils::IteratorUtils;
use advent_lib::*;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[parse_from(())]
struct Input {
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

fn calculate_part1(input: &Input) -> u64 {
    input
        .secrets
        .iter()
        .map(|&s| secrets(s).take(2001).last().unwrap() as u64)
        .sum()
}
fn calculate_part2(input: &Input) -> i32 {
    input
        .secrets
        .par_iter()
        .map(|&s| find_possible_price_changes(s))
        .reduce(FxHashMap::default, combine_prices)
        .values()
        .copied()
        .max()
        .unwrap()
}

day_main!();
day_test!( 22, example1 => 37327623, 24 );
day_test!( 22, example2 => 37990510, 23 );
day_test!( 22 => 16619522798, 1854 );
