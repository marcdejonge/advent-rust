/*
--- Day 5: If You Give A Seed A Fertilizer ---

You take the boat and find the gardener right where you were told he would be: managing a giant
"garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that Snow Island isn't receiving
any water.

"Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with
dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few
days... weeks... oh no." His face sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely forgot to check why we
stopped getting more sand! There's a ferry leaving soon that is headed over in that direction -
it's much faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another. "While you wait for the
ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just
arrived and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what
type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil,
what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil,
fertilizer and so on is identified with a number, but numbers are reused by each category - that is,
soil 123 and fertilizer 123 aren't necessarily related to each other.

The almanac starts by listing which seeds need to be planted.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source
category into numbers in a destination category. That is, the section that starts with seed-to-soil
map: describes how to convert a seed number (the source) to a soil number (the destination). This
lets the gardener and his team know which soil to use with which seeds, which water to use with
which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps
describe entire ranges of numbers that can be converted. Each line within a map contains three
numbers: the destination range start, the source range start, and the range length.

Any source numbers that aren't mapped correspond to the same destination number.

The gardener and his team want to get started as soon as possible, so they'd like to know the
closest location that needs a seed. Using these maps, find the lowest location number that
corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through
other categories until you can find its corresponding location number.

What is the lowest location number that corresponds to any of the initial seed numbers?
 */
#![feature(test)]
#![feature(iter_array_chunks)]

use advent_lib::day::*;
use prse_derive::parse;
use rayon::prelude::*;
use std::ops::Range;
use std::str::FromStr;

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
                let [left, middle, right] = change.map_range(&todo_range);
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

impl FromStr for ChangeDefinition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrs = s.split(' ').filter_map(|nr| nr.parse::<i64>().ok()).collect::<Vec<_>>();
        Ok(ChangeDefinition { in_range: nrs[1]..nrs[1] + nrs[2], change: nrs[0] - nrs[1] })
    }
}

impl ExecutableDay for Day {
    type Output = i64;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let seeds = parse!(lines.next().unwrap(), "seeds: {: :}");
        parse!(lines.next().unwrap(), "");

        let mut mappings = Vec::new();
        while lines.next().is_some() {
            let mut mapping = Mapping { changes: Vec::new() };
            loop {
                match lines.next() {
                    None => break,
                    Some(line) => {
                        if line.is_empty() {
                            break;
                        } else {
                            mapping.changes.push(line.parse().unwrap())
                        }
                    }
                }
            }
            mappings.push(mapping);
        }

        Day { seeds, mappings }
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
