#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::ZipWithNextTrait;
use rayon::prelude::*;
use std::ops::RangeInclusive;

struct Day {
    reports: Vec<Vec<i32>>,
}

const UP: RangeInclusive<i32> = 1..=3;
const DOWN: RangeInclusive<i32> = -3..=-1;

fn is_safe(report: &&Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }
    let mut diffs = report.iter().zip_with_next().map(|(a, b)| b - a);
    match diffs.next().unwrap() {
        s if UP.contains(&s) => diffs.all(|d| UP.contains(&d)),
        s if DOWN.contains(&s) => diffs.all(|d| DOWN.contains(&d)),
        _ => false,
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            reports: lines
                .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
                .collect(),
        }
    }
    fn calculate_part1(&self) -> Self::Output { self.reports.par_iter().filter(is_safe).count() }
    fn calculate_part2(&self) -> Self::Output {
        self.reports
            .par_iter()
            .filter(|report| {
                if is_safe(report) {
                    return true;
                }

                for remove_ix in 0..report.len() {
                    let report: Vec<i32> = report
                        .iter()
                        .enumerate()
                        .filter(|(ix, _)| ix != &remove_ix)
                        .map(|(_, nr)| *nr)
                        .collect();
                    if is_safe(&&report) {
                        return true;
                    }
                }

                false
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
