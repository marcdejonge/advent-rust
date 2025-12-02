#![feature(test)]

use advent_lib::*;
use fxhash::FxHashSet;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[parse_from(separated_list0(",", separated_pair(u64, "-", u64)))]
struct Input {
    ranges: Vec<(u64, u64)>,
}

fn decimal_digits(nr: u64) -> usize { nr.ilog10() as usize + 1 }

fn overlap_finder(
    block_count: usize,
    &(range_start, range_end): &(u64, u64),
) -> impl Iterator<Item = u64> {
    assert!(block_count >= 2);
    (decimal_digits(range_start).div_ceil(block_count)
        ..=decimal_digits(range_end).div_ceil(block_count))
        .flat_map(move |block_len| {
            let block_multiplier = 10u64.pow(block_len as u32); // e.g. 100 for block_len 2
            let mut step_size = 1;
            for _ in 1..block_count {
                step_size = step_size * block_multiplier + 1;
            }

            let min_mult = block_multiplier / 10; // Smallest multiplyer to get the right number of digits (e.g. 100)
            let max_mult = block_multiplier - 1; // Largest multiplyer to get the right number of digits (e.g. 999)
            let start = (step_size * min_mult).max(range_start.div_ceil(step_size) * step_size);
            let end = (step_size * max_mult).min((range_end / step_size) * step_size);

            (start..=end).step_by(step_size as usize)
        })
}

#[inline(never)]
fn calculate_part1(input: &Input) -> u64 {
    input.ranges.iter().flat_map(|range| overlap_finder(2, range)).sum()
}

#[inline(never)]
fn calculate_part2(input: &Input) -> u64 {
    let max_decimals = input.ranges.iter().map(|r| decimal_digits(r.1)).max().unwrap();
    input
        .ranges
        .par_iter()
        .map(|range| {
            (2..max_decimals)
                .flat_map(move |blocks| overlap_finder(blocks, range))
                .collect::<FxHashSet<_>>()
                .iter()
                .sum::<u64>()
        })
        .sum()
}

day_main!(Input);

day_test!( 2, example => 1227775554, 4174379265 );
day_test!( 2 => 19605500130, 36862281418 );
