#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::double_line_ending;
use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;

struct Day {
    ordering_rules: FxHashSet<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

impl Day {
    fn calculate_middle(&self, use_changed: bool) -> u32 {
        self.pages
            .iter()
            .filter_map(|update| {
                let mut new = update.clone();
                new.sort_by(|&a, &b| {
                    if self.ordering_rules.contains(&(a, b)) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });

                if new.eq(update) ^ use_changed {
                    Some(new)
                } else {
                    None
                }
            })
            .map(|update| update[update.len() / 2])
            .sum()
    }
}

impl ExecutableDay for Day {
    type Output = u32;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(
                separated_list1(
                    line_ending,
                    separated_pair(complete::u32, tag(b"|"), complete::u32),
                ),
                double_line_ending,
                separated_list1(line_ending, separated_list1(tag(b","), complete::u32)),
            ),
            |(ordering_rules, pages)| Day {
                ordering_rules: ordering_rules.into_iter().collect(),
                pages,
            },
        )
    }

    fn calculate_part1(&self) -> Self::Output { self.calculate_middle(false) }

    fn calculate_part2(&self) -> Self::Output { self.calculate_middle(true) }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 5, example1 => 143, 123 );
    day_test!( 5 => 5955, 4030);
}
