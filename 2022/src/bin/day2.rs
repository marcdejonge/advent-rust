#![feature(test)]

use advent_lib::parsing::parsable_pair;
use advent_lib::*;
use nom_parse_macros::parse_from;
use Left::*;
use Right::*;

#[parse_from(separated_list1(line_ending, parsable_pair(space1)))]
struct Input {
    input: Vec<(Left, Right)>,
}

#[parse_from]
#[derive(Clone, Copy)]
enum Left {
    #[format("A")]
    A,
    #[format("B")]
    B,
    #[format("C")]
    C,
}

#[parse_from]
#[derive(Clone, Copy)]
enum Right {
    #[format("X")]
    X,
    #[format("Y")]
    Y,
    #[format("Z")]
    Z,
}

fn calculate_part1(input: &Input) -> i32 {
    input
        .input
        .iter()
        .map(|line| match line {
            (A, X) => 4,
            (A, Y) => 8,
            (A, Z) => 3,
            (B, X) => 1,
            (B, Y) => 5,
            (B, Z) => 9,
            (C, X) => 7,
            (C, Y) => 2,
            (C, Z) => 6,
        })
        .sum()
}

fn calculate_part2(input: &Input) -> i32 {
    input
        .input
        .iter()
        .map(|line| match line {
            (A, X) => 3,
            (A, Y) => 4,
            (A, Z) => 8,
            (B, X) => 1,
            (B, Y) => 5,
            (B, Z) => 9,
            (C, X) => 2,
            (C, Y) => 6,
            (C, Z) => 7,
        })
        .sum()
}

day_main!();
day_test!( 2, example => 15, 12 );
day_test!( 2 => 13565, 12424 );
