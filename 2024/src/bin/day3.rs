#![feature(test)]
extern crate core;

use advent_lib::parsing::find_many_skipping_unknown;
use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from]
#[derive(Debug, PartialEq, Eq)]
enum Command {
    #[format(delimited("mul(", separated_pair(i64, ",", i64), ")"))]
    Mul(i64, i64),
    #[format("do()")]
    Do,
    #[format("don't()")]
    Dont,
}

#[parse_from(find_many_skipping_unknown())]
#[derive(Debug)]
struct Memory {
    commands: Vec<Command>,
}

fn calculate_part1(memory: &Memory) -> i64 {
    memory
        .commands
        .iter()
        .map(|command| if let Command::Mul(a, b) = command { a * b } else { 0 })
        .sum()
}

fn calculate_part2(memory: &Memory) -> i64 {
    memory
        .commands
        .iter()
        .fold((0, true), |(sum, enabled), command| match command {
            Command::Mul(a, b) if enabled => (sum + a * b, enabled),
            Command::Do => (sum, true),
            Command::Dont => (sum, false),
            _ => (sum, enabled),
        })
        .0
}

day_main!(Memory);
day_test!( 3, example1 => 161, 161 );
day_test!( 3, example2 => 161, 48 );
day_test!( 3 => 171183089, 63866497);
