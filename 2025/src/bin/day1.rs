#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from]
struct Input {
    dummy: Vec<u32>,
}

fn calculate_part1(input: &Input) -> u64 { 0 }

fn calculate_part2(input: &Input) -> u64 { 0 }

day_main!(Input);

day_test!( 1, example => 0,0 );
day_test!( 1 => 0, 0 );
