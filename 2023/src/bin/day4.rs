#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::{multi_line_parser, Parsable};
use fxhash::FxHashSet;
use nom::bytes::streaming::tag;
use nom::character::complete;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::Parser;
use std::ops::Shl;

struct Day {
    cards: Vec<Card>,
}

struct Card {
    winning: FxHashSet<u8>,
    drawn: Vec<u8>,
}

impl Parsable for Card {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            preceded(
                tuple((tag(b"Card"), space1, complete::u64, tag(b":"), space1)),
                separated_pair(
                    separated_list1(space1, complete::u8),
                    tuple((tag(b" |"), space1)),
                    separated_list1(space1, complete::u8),
                ),
            ),
            |(winning, drawn)| Card { winning: winning.into_iter().collect(), drawn },
        )
    }
}

impl Card {
    fn winning_count(&self) -> usize {
        self.drawn.iter().filter(|n| self.winning.contains(n)).count()
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(multi_line_parser(), |cards| Day { cards })
    }

    fn calculate_part1(&self) -> Self::Output {
        self.cards
            .iter()
            .map(|c| {
                let count = c.winning_count();
                if count == 0 {
                    0
                } else {
                    1usize.shl(count - 1)
                }
            })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut counts = Vec::with_capacity(self.cards.len());
        for _ in 0..self.cards.len() {
            counts.push(1usize)
        }

        self.cards.iter().enumerate().for_each(|(ix, c)| {
            let curr_count = counts[ix];
            for next_ix in ix + 1..=ix + c.winning_count() {
                counts[next_ix] += curr_count;
            }
        });
        counts.iter().sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example => 13, 30 );
    day_test!( 4 => 18519, 11787590);
}
