#![feature(test)]

use std::ops::Add;

use fxhash::FxHashMap;
use prse_derive::parse;
use rayon::prelude::*;

use advent_lib::day::*;

use crate::Spring::{Broken, Operational, Unknown};

struct Day {
    lines: Vec<(Vec<Spring>, Vec<usize>)>,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Spring {
    Unknown,
    Operational,
    Broken,
}

fn parse_springs(line: &str) -> Vec<Spring> {
    line.chars()
        .map(|c| match c {
            '.' => Operational,
            '#' => Broken,
            '?' => Unknown,
            _ => panic!("Unknown state {c}"),
        })
        .collect()
}

fn find_options(
    springs: &[Spring],
    counts: &[usize],
    memoize: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    let springs_len = springs.len();
    let counts_len = counts.len();

    if springs_len == 0 {
        return if counts_len == 0 { 1 } else { 0 };
    } else if counts_len == 0 {
        return if springs.iter().all(|spring| *spring != Broken) { 1 } else { 0 };
    } else if let Some(result) = memoize.get(&(springs_len, counts_len)) {
        return *result;
    }

    let mut result = 0;

    if springs[0] == Operational || springs[0] == Unknown {
        result += find_options(&springs[1..], counts, memoize);
    }

    if springs[0] == Broken || springs[0] == Unknown {
        let next_count = counts[0];
        result += if springs_len == next_count
            && counts_len == 1
            && springs.iter().all(|spring| *spring != Operational)
        {
            1
        } else if springs_len > next_count
            && springs[0..next_count].iter().all(|x| *x != Operational)
            && springs[next_count] != Broken
        {
            find_options(&springs[next_count + 1..], &counts[1..], memoize)
        } else {
            0
        }
    }

    memoize.insert((springs_len, counts_len), result);
    result
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let lines = lines
            .map(|line| {
                let (line, counts): (String, Vec<_>) = parse!(line, "{} {:,:}");
                (parse_springs(&line), counts)
            })
            .collect::<Vec<_>>();

        Day { lines }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.lines
            .par_iter()
            .map(|(line, nrs)| {
                let mut memoize = FxHashMap::default();
                find_options(line.as_slice(), nrs.as_slice(), &mut memoize)
            })
            .reduce(|| 0, usize::add)
    }

    fn calculate_part2(&self) -> Self::Output {
        self.lines
            .par_iter()
            .map(|(line, nrs)| {
                let mut long_line = Vec::with_capacity(5 * (line.len() + 1) - 1);
                for ix in 0..5 {
                    if ix > 0 {
                        long_line.push(Unknown);
                    }
                    line.iter().for_each(|&v| long_line.push(v));
                }
                (long_line, nrs.repeat(5))
            })
            .map(|(line, nrs)| {
                let mut memoize = FxHashMap::default();
                find_options(line.as_slice(), nrs.as_slice(), &mut memoize)
            })
            .reduce(|| 0, usize::add)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 12, example => 21, 525152 );
    day_test!( 12 => 7633, 23903579139437);
}
