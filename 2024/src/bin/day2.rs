#![feature(test)]

use advent_lib::day_main;
use advent_lib::iter_utils::IteratorUtils;
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use std::ops::RangeInclusive;

#[parse_from(separated_list1(line_ending, separated_list1(space1, i32)))]
struct Input {
    reports: Vec<Vec<i32>>,
}

const UP: RangeInclusive<i32> = 1..=3;
const DOWN: RangeInclusive<i32> = -3..=-1;

fn find_unsafe_index(report: &&Vec<i32>) -> Option<usize> {
    if report.len() < 2 {
        return None;
    }
    let mut diffs = report.iter().zip_with_next().map(|(a, b)| b - a).enumerate();
    match diffs.next().unwrap() {
        (_, s) if UP.contains(&s) => diffs.find(|(_, d)| !UP.contains(d)).map(|(ix, _)| ix),
        (_, s) if DOWN.contains(&s) => diffs.find(|(_, d)| !DOWN.contains(d)).map(|(ix, _)| ix),
        _ => Some(0),
    }
}

fn remove_index(report: &[i32], remove_ix: usize) -> Vec<i32> {
    report
        .iter()
        .enumerate()
        .filter(|(ix, _)| ix != &remove_ix)
        .map(|(_, nr)| *nr)
        .collect()
}

fn calculate_part1(input: &Input) -> usize {
    input
        .reports
        .par_iter()
        .filter(|report| find_unsafe_index(report).is_none())
        .count()
}

fn calculate_part2(input: &Input) -> usize {
    input.reports
            .par_iter()
            .filter(|report| match find_unsafe_index(report) {
                None => true,
                Some(remove_ix) => {
                    // Either the first or second number can be removed
                    find_unsafe_index(&&remove_index(report, remove_ix)).is_none()
                        || find_unsafe_index(&&remove_index(report, remove_ix + 1)).is_none()
                        // Or if it finds it at the beginning, the start might be wrong
                        || (remove_ix == 1 && find_unsafe_index(&&remove_index(report, 0)).is_none())
                }
            })
            .count()
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example1 => 2, 4 );
    day_test!( 2 => 314, 373);
}
