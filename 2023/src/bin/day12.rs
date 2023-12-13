/*
--- Day 12: Hot Springs ---

You finally reach the hot springs! You can see steam rising from secluded areas attached to the
primary, ornate building.

As you turn to enter, the researcher stops you. "Wait - I thought you were looking for the hot
springs, weren't you?" You indicate that this definitely looks like hot springs to you.

"Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."

You look in the direction the researcher is pointing and suddenly notice the massive metal helixes
towering overhead. "This way!"

It only takes you a few more steps to reach the main gate of the massive fenced-off area containing
the springs. You go through the gate and into a small administrative building.

"Hello! What brings you to the hot springs today? Sorry they're not very hot right now; we're having
a lava shortage at the moment." You ask about the missing machine parts for Desert Island.

"Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment, not until
we get more lava to heat our forges. And our springs. The springs aren't very springy unless they're hot!"

"Say, could you go up and see why the lava stopped flowing? The springs are too cold for normal
operation, but we should be able to find one springy enough to launch you up there!"

There's just one problem - many of the springs have fallen into disrepair, so they're not actually
sure which springs would even be safe to use! Worse yet, their condition records of which springs
are damaged (your puzzle input) are also damaged! You'll need to help them repair the damaged
records.

In the giant field just outside, the springs are arranged into rows. For each row, the condition
records show every spring and whether it is operational (.) or damaged (#). This is the part of the
condition records that is itself damaged; for some springs, it is simply unknown (?) whether the
spring is operational or damaged.

However, the engineer that produced the condition records also duplicated some of this information
in a different format! After the list of springs for a given row, the size of each contiguous group
of damaged springs is listed in the order those groups appear in the row. This list always accounts
for every damaged spring, and each number is the entire size of its contiguous group (that is,
groups are always separated by at least one operational spring: #### would always be 4, never 2,2).

For each row, count all of the different arrangements of operational and broken springs that meet
the given criteria. What is the sum of those counts?

--- Part Two ---

As you look out at the field of springs, you feel like there are way more springs than the condition
records list. When you examine the records, you discover that they were actually folded up this
whole time!

To unfold the records, on each row, replace the list of spring conditions with five copies of itself
(separated by ?) and replace the list of contiguous groups of damaged springs with five copies of
itself (separated by ,).

Unfold your condition records; what is the new sum of possible arrangement counts?

*/
#![feature(test)]

use crate::Spring::{Broken, Operational, Unknown};
use advent_lib::day::{execute_day, ExecutableDay};
use fxhash::FxHashMap;
use prse_derive::parse;
use rayon::prelude::*;
use std::ops::Add;

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
