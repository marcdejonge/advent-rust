#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use nom::character::complete;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Parser;
use rayon::prelude::*;
use std::ops::RangeInclusive;

struct Day {
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
        (_, s) if UP.contains(&s) => diffs.find(|(_, d)| !UP.contains(&d)).map(|(ix, _)| ix),
        (_, s) if DOWN.contains(&s) => diffs.find(|(_, d)| !DOWN.contains(&d)).map(|(ix, _)| ix),
        _ => Some(0),
    }
}

fn remove_index(report: &&Vec<i32>, remove_ix: usize) -> Vec<i32> {
    report
        .iter()
        .enumerate()
        .filter(|(ix, _)| ix != &remove_ix)
        .map(|(_, nr)| *nr)
        .collect()
}

impl ExecutableDay for Day {
    type Output = usize;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                complete::line_ending::<&[u8], _>,
                separated_list1(complete::space1, complete::i32),
            ),
            |reports| Day { reports },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        self.reports
            .par_iter()
            .filter(|report| find_unsafe_index(report).is_none())
            .count()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.reports
            .par_iter()
            .filter(|report| match find_unsafe_index(report) {
                None => true,
                Some(remove_ix) => {
                    // Either the first or second number can be removed
                    find_unsafe_index(&&remove_index(report, remove_ix)) == None
                        || find_unsafe_index(&&remove_index(report, remove_ix + 1)) == None
                        // Or if it finds it at the beginning, the start might be wrong
                        || (remove_ix == 1 && find_unsafe_index(&&remove_index(report, 0)) == None)
                }
            })
            .count()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example1 => 2, 4 );
    day_test!( 2 => 314, 373);
}
