#![feature(test)]

use advent_lib::day_main;
use advent_macros::parsable;
use std::collections::BinaryHeap;

#[parsable(separated_list1(double_line_ending, separated_list1(line_ending, i32)))]
struct Input {
    sums: Vec<Vec<i32>>,
}

fn sums(input: &Input) -> Vec<i32> {
    input
        .sums
        .iter()
        .map(|v| v.iter().sum())
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
}

fn calculate_part1(input: &Input) -> i32 { sums(input).iter().rev().take(1).sum() }

fn calculate_part2(input: &Input) -> i32 { sums(input).iter().rev().take(3).sum() }

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example => 24000, 45000 );
    day_test!( 1 => 68292, 203203);
}
