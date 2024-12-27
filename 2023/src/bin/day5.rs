#![feature(test)]
#![feature(iter_array_chunks)]

use advent_lib::day::*;
use advent_lib::parsing::{double_line_ending, multi_line_parser, Parsable};
use nom::bytes::complete::{tag, take_while};
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::Parser;
use rayon::prelude::*;
use std::ops::Range;

struct Day {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct ChangeDefinition {
    in_range: Range<i64>,
    change: i64,
}

impl ChangeDefinition {
    fn map_range(&self, value: &Range<i64>) -> [Option<Range<i64>>; 3] {
        if value.start < self.in_range.start {
            if value.end < self.in_range.start {
                [Some(value.clone()), None, None]
            } else if value.end > self.in_range.end {
                [
                    Some(value.start..self.in_range.start),
                    Some(self.in_range.start + self.change..self.in_range.end + self.change),
                    Some(self.in_range.end..value.end),
                ]
            } else {
                [
                    Some(value.start..self.in_range.start),
                    Some(self.in_range.start + self.change..value.end + self.change),
                    None,
                ]
            }
        } else if value.start >= self.in_range.end {
            [None, None, Some(value.clone())]
        } else if value.end <= self.in_range.end {
            [
                None,
                Some(value.start + self.change..value.end + self.change),
                None,
            ]
        } else {
            [
                None,
                Some(value.start + self.change..self.in_range.end + self.change),
                Some(self.in_range.end..value.end),
            ]
        }
    }
}

impl Parsable for ChangeDefinition {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(separated_list1(space1, complete::i64), |nrs| {
            ChangeDefinition { in_range: nrs[1]..nrs[1] + nrs[2], change: nrs[0] - nrs[1] }
        })
    }
}

struct Mapping {
    changes: Vec<ChangeDefinition>,
}

impl Mapping {
    fn map_single(&self, value: i64) -> i64 {
        for change in &self.changes {
            if change.in_range.contains(&value) {
                return value + change.change;
            }
        }

        value
    }

    fn map_range(&self, ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut result = Vec::new();
        let mut todo = ranges;
        for change in &self.changes {
            let mut next_todo = Vec::new();
            for todo_range in &todo {
                let [left, middle, right] = change.map_range(todo_range);
                if let Some(left) = left {
                    next_todo.push(left)
                }
                if let Some(middle) = middle {
                    result.push(middle)
                }
                if let Some(right) = right {
                    next_todo.push(right);
                }
            }
            todo = next_todo
        }

        todo.into_iter().for_each(|range| result.push(range));
        result
    }
}

impl ExecutableDay for Day {
    type Output = i64;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(
                preceded(tag(b"seeds: "), separated_list1(space1, complete::i64)),
                double_line_ending,
                separated_list1(
                    double_line_ending,
                    map(
                        preceded(
                            tuple((take_while(|b| b != b' '), tag(b" map:"), line_ending)),
                            multi_line_parser(),
                        ),
                        |changes| Mapping { changes },
                    ),
                ),
            ),
            |(seeds, mappings)| Day { seeds, mappings },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        self.seeds
            .iter()
            .map(|seed| self.mappings.iter().fold(*seed, |curr, mapping| mapping.map_single(curr)))
            .min()
            .unwrap()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.seeds
            .iter()
            .array_chunks::<2>()
            .par_bridge()
            .map(|[&start, &count]| start..start + count)
            .flat_map(|seed_range| {
                self.mappings
                    .iter()
                    .fold(vec![seed_range], |curr, mapping| mapping.map_range(curr))
                    .iter()
                    .map(|v| v.start)
                    .collect::<Vec<_>>()
            })
            .min()
            .unwrap()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 5, example => 35, 46 );
    day_test!( 5 => 910845529, 77435348);
}
