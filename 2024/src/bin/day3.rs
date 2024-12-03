#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::find_many;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Mul(i32, i32),
    Do,
    Dont,
}

fn number_parser(input: &str) -> IResult<&str, i32> {
    map_res(take_while(|c: char| c.is_ascii_digit()), str::parse)(input)
}

fn command_parser(input: &str) -> IResult<&str, Command> {
    alt((
        map(
            delimited(
                tag("mul("),
                separated_pair(number_parser, tag(","), number_parser),
                tag(")"),
            ),
            |(a, b)| Command::Mul(a, b),
        ),
        map(tag("do()"), |_| Command::Do),
        map(tag("don't()"), |_| Command::Dont),
    ))(input)
}

#[derive(Debug)]
struct Day {
    commands: Vec<Command>,
}

impl ExecutableDay for Day {
    type Output = i32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { commands: lines.flat_map(|line| find_many(command_parser, &line)).collect() }
    }
    fn calculate_part1(&self) -> Self::Output {
        self.commands
            .iter()
            .map(|command| if let Command::Mul(a, b) = command { a * b } else { 0 })
            .sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut sum = 0;
        let mut enabled = true;
        for command in &self.commands {
            match command {
                Command::Mul(a, b) if enabled => sum += a * b,
                Command::Do => enabled = true,
                Command::Dont => enabled = false,
                _ => {}
            }
        }
        sum
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
