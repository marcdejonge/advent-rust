#![feature(test)]

use advent_lib::{parsing::double_line_ending, *};
use nom_parse_macros::parse_from;
use std::ops::RangeInclusive;

#[parse_from(separated_pair(
    separated_list1(newline, map(separated_pair(u64, tag("-"), u64), |(start, end)| start..=end)),
    double_line_ending,
    separated_list1(newline, u64)
))]
struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ingriedients: Vec<u64>,
}

fn calculate_part1(input: &Input) -> usize {
    input
        .ingriedients
        .iter()
        .filter(|&&ingriedient| input.ranges.iter().any(|range| range.contains(&ingriedient)))
        .count()
}

fn calculate_part2(input: &Input) -> usize {
    let mut ranges = input.ranges.clone();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    0
}

day_main!(Input);

day_test!( 5, example => 3, 14 );
day_test!( 5 => 640, 0 );
