#![feature(test)]

use std::ops::Range;

use advent_lib::{iter_utils::SumWith, *};
use nom_parse_macros::parse_from;

#[parse_from(separated_list1(",", i32))]
struct Input {
    crab_positions: Vec<i32>,
    #[derived(*crab_positions.iter().min().unwrap()..*crab_positions.iter().max().unwrap())]
    possible_positions: Range<i32>,
}

impl Input {
    fn calc(&self, crab_fn: fn(i32) -> i32) -> i32 {
        self.possible_positions
            .clone()
            .map(|pos| self.crab_positions.iter().sum_with(|it| crab_fn(it - pos)))
            .min()
            .unwrap()
    }
}

fn calculate_part1(input: &Input) -> i32 {
    input.calc(i32::abs)
}

fn calculate_part2(input: &Input) -> i32 {
    input.calc(|nr| (nr.abs() * (nr.abs() + 1)) / 2)
}

day_main!();

day_test!( 7, example => 37, 168 );
day_test!( 7 => 351901, 101079875 );
