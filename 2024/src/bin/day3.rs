#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::find_many_skipping_unknown;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::{map, map_res};
use nom::error::{Error, FromExternalError, ParseError};
use nom::sequence::{delimited, separated_pair};
use nom::Parser;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Mul(i64, i64),
    Do,
    Dont,
}

fn number_parser<'a, E: FromExternalError<&'a [u8], ParseIntError> + ParseError<&'a [u8]>>(
) -> impl Parser<&'a [u8], i64, E> {
    map_res(take_while_m_n(1, 3, |b: u8| b.is_ascii_digit()), |bs| {
        std::str::from_utf8(bs).unwrap().parse::<i64>()
    })
}

fn command_parser<'a, E: FromExternalError<&'a [u8], ParseIntError> + ParseError<&'a [u8]>>(
) -> impl Parser<&'a [u8], Command, E> {
    alt((
        map(
            delimited(
                tag(b"mul("),
                separated_pair(number_parser(), tag(b","), number_parser()),
                tag(b")"),
            ),
            |(a, b)| Command::Mul(a, b),
        ),
        map(tag(b"do()"), |_| Command::Do),
        map(tag(b"don't()"), |_| Command::Dont),
    ))
}

#[derive(Debug)]
struct Day {
    commands: Vec<Command>,
}

impl ExecutableDay for Day {
    type Output = i64;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(find_many_skipping_unknown(command_parser()), |commands| {
            Day { commands }
        })
    }

    fn calculate_part1(&self) -> Self::Output {
        self.commands
            .iter()
            .map(|command| if let Command::Mul(a, b) = command { a * b } else { 0 })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.commands
            .iter()
            .fold((0, true), |(sum, enabled), command| match command {
                Command::Mul(a, b) if enabled => (sum + a * b, enabled),
                Command::Do => (sum, true),
                Command::Dont => (sum, false),
                _ => (sum, enabled),
            })
            .0
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 3, example1 => 161, 161 );
    day_test!( 3, example2 => 161, 48 );
    day_test!( 3 => 171183089, 63866497);
}
