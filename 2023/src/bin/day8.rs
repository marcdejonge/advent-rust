#![feature(test)]

use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::Parser;
use num::integer::lcm;
use rayon::prelude::*;

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::key::Key;
use advent_lib::parsing::{double_line_ending, Parsable};

struct Day {
    instructions: Vec<Turn>,
    steps: FxHashMap<Key, (Key, Key)>,
}

enum Turn {
    Left,
    Right,
}

impl Day {
    fn next_step(&self, curr: Key, turn: &Turn) -> Option<Key> {
        let next_steps = self.steps.get(&curr);
        let next = if let Some((left, right)) = next_steps {
            match turn {
                Turn::Left => *left,
                Turn::Right => *right,
            }
        } else {
            return None;
        };
        if curr == next {
            None
        } else {
            Some(next)
        }
    }

    fn walk(&self, start: Key) -> impl Iterator<Item = Key> + '_ {
        self.instructions.iter().repeat().scan(start, |curr, turn| {
            if let Some(next) = self.next_step(*curr, turn) {
                *curr = next;
                Some(next)
            } else {
                None
            }
        })
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(
                many1(alt((
                    map(tag(b"L"), |_| Turn::Left),
                    map(tag(b"R"), |_| Turn::Right),
                ))),
                double_line_ending,
                separated_list1(
                    line_ending,
                    separated_pair(
                        Key::parser(),
                        tag(b" = "),
                        delimited(
                            tag(b"("),
                            separated_pair(Key::parser(), tag(b", "), Key::parser()),
                            tag(b")"),
                        ),
                    ),
                ),
            ),
            |(instructions, steps)| Day { instructions, steps: steps.into_iter().collect() },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        const TARGET: Key = Key::fixed(b"zzz");
        self.walk(Key::fixed(b"aaa")).take_while(|&c| c != TARGET).count() + 1
    }

    fn calculate_part2(&self) -> Self::Output {
        let end_steps: Vec<_> = self
            .steps
            .par_iter()
            .filter(|(p, _)| p.last_char() == b'a')
            .map(|(start, _)| {
                for (step, place) in self.walk(*start).enumerate() {
                    if place.last_char() == b'z' {
                        return step + 1;
                    }
                }
                0
            })
            .collect();

        end_steps.iter().fold(1, |curr, next| lcm(curr, *next))
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 8, example1 => 2, 2);
    day_test!( 8, example2 => 6, 6);
    day_test!( 8, example3 => 1, 6);
    day_test!( 8 => 12361, 18215611419223);
}
