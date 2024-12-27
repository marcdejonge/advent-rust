#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::{multi_line_parser, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::Parser;
use std::cmp::max;

struct Day {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    index: u64,
    draws: Vec<Draw>,
}

impl Parsable for Game {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(
                preceded(tag(b"Game "), complete::u64),
                tag(b": "),
                separated_list1(tag(b"; "), Draw::parser()),
            ),
            |(index, draws)| Game { index, draws },
        )
    }
}

#[derive(Debug)]
struct Draw {
    red: u64,
    green: u64,
    blue: u64,
}

impl Draw {
    fn empty() -> Self { Draw { red: 0, green: 0, blue: 0 } }
    fn power(&self) -> u64 { self.red * self.green * self.blue }
    fn red(count: u64) -> Self { Draw { red: count, green: 0, blue: 0 } }
    fn green(count: u64) -> Self { Draw { red: 0, green: count, blue: 0 } }
    fn blue(count: u64) -> Self { Draw { red: 0, green: 0, blue: count } }
}

impl Parsable for Draw {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                tag(b", "),
                alt((
                    map(terminated(complete::u64, tag(b" red")), |count| {
                        Draw::red(count)
                    }),
                    map(terminated(complete::u64, tag(b" green")), |count| {
                        Draw::green(count)
                    }),
                    map(terminated(complete::u64, tag(b" blue")), |count| {
                        Draw::blue(count)
                    }),
                )),
            ),
            |draws| {
                draws.iter().fold(Draw::empty(), |curr, next| Draw {
                    red: curr.red + next.red,
                    green: curr.green + next.green,
                    blue: curr.blue + next.blue,
                })
            },
        )
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(multi_line_parser(), |games| Day { games })
    }

    fn calculate_part1(&self) -> Self::Output {
        self.games
            .iter()
            .filter(|game| {
                game.draws
                    .iter()
                    .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
            })
            .map(|game| game.index)
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.games
            .iter()
            .map(|game| {
                game.draws
                    .iter()
                    .fold(Draw::empty(), |curr, next| Draw {
                        red: max(curr.red, next.red),
                        green: max(curr.green, next.green),
                        blue: max(curr.blue, next.blue),
                    })
                    .power()
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example => 8, 2286 );
    day_test!( 2 => 2716, 72227 );
}
