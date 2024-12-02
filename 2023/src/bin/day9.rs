#![feature(test)]
#![feature(iter_collect_into)]

use rayon::prelude::*;

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;

struct Day {
    input: Vec<Vec<i64>>,
}

fn calc_next(nrs: &Vec<i64>) -> i64 {
    if let Some(&last) = nrs.last() {
        if nrs.iter().all(|&nr| nr == last) {
            last
        } else {
            last + calc_next(
                nrs.iter()
                    .zip_with_next()
                    .map(|(x, y)| y - x)
                    .collect_into(&mut Vec::with_capacity(nrs.len() - 1)),
            )
        }
    } else {
        0
    }
}

fn calc_prev(nrs: &Vec<i64>) -> i64 {
    if let Some(&first) = nrs.first() {
        if nrs.iter().all(|&nr| nr == first) {
            first
        } else {
            first
                - calc_prev(
                    nrs.iter()
                        .zip_with_next()
                        .map(|(x, y)| y - x)
                        .collect_into(&mut Vec::with_capacity(nrs.len() - 1)),
                )
        }
    } else {
        0
    }
}

impl ExecutableDay for Day {
    type Output = i64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            input: lines
                .map(|line| line.split(' ').filter_map(|nr| nr.parse().ok()).collect())
                .collect(),
        }
    }

    fn calculate_part1(&self) -> Self::Output { self.input.par_iter().map(calc_next).sum() }

    fn calculate_part2(&self) -> Self::Output { self.input.par_iter().map(calc_prev).sum() }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 9, example => 114, 2);
    day_test!( 9 => 1995001648, 988 );
}
