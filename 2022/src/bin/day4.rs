#![feature(test)]
#![allow(clippy::ptr_arg)]

use advent_lib::parsing::parsable_pair;
use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(parsable_pair("-"))]
struct Range {
    from: u32,
    to: u32,
}

#[parse_from(parsable_pair(","))]
struct RangePair(Range, Range);

impl Range {
    fn contains(&self, value: u32) -> bool { value >= self.from && value <= self.to }
    fn wraps(&self, other: &Range) -> bool { self.from <= other.from && self.to >= other.to }
}

fn calculate_part1(range_pair: &[RangePair]) -> usize {
    range_pair
        .iter()
        .filter(|RangePair(first, second)| first.wraps(second) || second.wraps(first))
        .count()
}

fn calculate_part2(range_pair: &[RangePair]) -> usize {
    range_pair
        .iter()
        .filter(|RangePair(first, second)| {
            first.contains(second.from)
                || first.contains(second.to)
                || second.contains(first.from)
                || second.contains(first.to)
        })
        .count()
}

day_main!(Vec<RangePair>);
day_test!( 4, example => 2, 4 );
day_test!( 4 => 580, 895 );
