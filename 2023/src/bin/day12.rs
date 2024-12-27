#![feature(test)]

use std::ops::Add;

use fxhash::FxHashMap;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::Parser;
use rayon::prelude::*;

use crate::Spring::{Broken, Operational, Unknown};
use advent_lib::day::*;
use advent_macros::FromRepr;

struct Day {
    lines: Vec<(Vec<Spring>, Vec<usize>)>,
}

#[repr(u8)]
#[derive(FromRepr, Eq, PartialEq, Debug, Copy, Clone)]
enum Spring {
    Unknown = b'?',
    Operational = b'.',
    Broken = b'#',
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

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                line_ending,
                separated_pair(
                    many1(Spring::parse),
                    space1,
                    separated_list1(tag(b","), map(complete::u64, |nr| nr as usize)),
                ),
            ),
            |lines| Day { lines },
        )
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
