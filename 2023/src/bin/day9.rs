/*
--- Day 9: Mirage Maintenance ---

You ride the camel through the sandstorm and stop where the ghost's maps told you to stop. The
sandstorm subsequently subsides, somehow seeing you standing at an oasis!

The camel goes to get some water and you stretch your neck. As you look up, you discover what must
be yet another giant floating island, this one made of metal! That must be where the parts to fix
the sand machines come from.

There's even a hang glider partially buried in the sand here; once the sun rises and heats up the
sand, you might be able to use the glider and the hot air to get all the way up to the metal island!

While you wait for the sun to rise, you admire the oasis hidden here in the middle of Desert Island.
It must have a delicate ecosystem; you might as well take some ecological readings while you wait.
Maybe you can report any environmental instabilities you find to someone so the oasis can be around
for the next sandstorm-worn traveler.

You pull out your handy Oasis And Sand Instability Sensor and analyze your surroundings. The OASIS
produces a report of many values and how they are changing over time (your puzzle input). Each line
in the report contains the history of a single value.

To best protect the oasis, your environmental report should include a prediction of the next value
in each history. To do this, start by making a new sequence from the difference at each step of your
history. If that sequence is not all zeroes, repeat this process, using the sequence you just
generated as the input sequence. Once all of the values in your latest sequence are zeroes, you can
extrapolate what the next value of the original history should be.

Analyze your OASIS report and extrapolate the next value for each history. What is the sum of these
extrapolated values?
 */

#![feature(test)]
#![feature(iter_collect_into)]

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::iter_utils::ZipWithNextTrait;

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

    fn calculate_part1(&self) -> Self::Output { self.input.iter().map(calc_next).sum() }

    fn calculate_part2(&self) -> Self::Output { self.input.iter().map(calc_prev).sum() }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 9, example => 114, 2);
    day_test!( 9 => 1995001648, 988 );
}
