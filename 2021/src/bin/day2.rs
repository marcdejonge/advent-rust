#![feature(test)]

use advent_lib::{day_main, day_test};
use nom_parse_macros::parse_from;

#[parse_from]
enum Command {
    #[format(preceded("forward ", i32))]
    Forward(i32),
    #[format(preceded("down ", i32))]
    Down(i32),
    #[format(preceded("up ", i32))]
    Up(i32),
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn get_result(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn calculate_part1(commands: &Vec<Command>) -> i32 {
    commands
        .iter()
        .fold(Position::default(), |mut pos, command| {
            match command {
                Command::Forward(x) => pos.horizontal += x,
                Command::Down(x) => pos.depth += x,
                Command::Up(x) => pos.depth -= x,
            }
            pos
        })
        .get_result()
}

fn calculate_part2(commands: &Vec<Command>) -> i32 {
    commands
        .iter()
        .fold(Position::default(), |mut pos, command| {
            match command {
                Command::Forward(x) => {
                    pos.horizontal += x;
                    pos.depth += pos.aim * x;
                }
                Command::Down(x) => pos.aim += x,
                Command::Up(x) => pos.aim -= x,
            }
            pos
        })
        .get_result()
}

day_main!();

day_test!( 2, example => 150, 900 );
day_test!( 2 => 1938402, 1947878632 );
