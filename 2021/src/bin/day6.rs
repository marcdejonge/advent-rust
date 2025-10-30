#![feature(test)]

use advent_lib::{iter_utils::SumWith, *};
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;

#[parse_from(separated_list1(",", i32))]
struct Input(Vec<i32>);

impl Input {
    fn count_all_fishes(&self, generation: i32) -> u64 {
        self.0.iter().sum_with(|s| count_fishes(generation + 8 - s))
    }
}

#[memoize::memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn count_fishes(gen: i32) -> u64 {
    1 + (0..=gen - 9).rev().step_by(7).sum_with(count_fishes)
}

fn calculate_part1(input: &Input) -> u64 {
    input.count_all_fishes(80)
}

fn calculate_part2(input: &Input) -> u64 {
    input.count_all_fishes(256)
}

day_main!();

day_test!( 6, example => 5934, 26984457539 );
day_test!( 6 => 379414, 1705008653296 );
