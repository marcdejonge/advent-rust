#![feature(test)]

use advent_lib::{day_main, day_test};

fn calculate_part1(input: &Vec<u32>) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn calculate_part2(input: &Vec<u32>) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

day_main!();

day_test!( 1, example => 7, 5 );
day_test!( 1 => 1681, 1704 );
