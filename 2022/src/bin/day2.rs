#![feature(test)]

use advent_lib::day_main;
use advent_macros::parsable;
use Left::*;
use Right::*;

#[parsable(separated_list1(line_ending, parsable_pair(space1)))]
struct Input {
    input: Vec<(Left, Right)>,
}

#[parsable]
#[derive(Clone, Copy)]
enum Left {
    A,
    B,
    C,
}

#[parsable]
#[derive(Clone, Copy)]
enum Right {
    X,
    Y,
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

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example => 15, 12 );
    day_test!( 2 => 13565, 12424 );
}
