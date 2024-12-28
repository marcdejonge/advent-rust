#![feature(test)]

use advent_lib::day::*;
use advent_macros::parsable;

#[derive(Debug, PartialEq, Eq)]
#[parsable]
enum Command {
    #[format=delimited(tag(b"mul("), separated_pair(i64, tag(b","), i64), tag(b")"))]
    Mul(i64, i64),
    #[format=tag(b"do()")]
    Do,
    #[format=tag(b"don't()")]
    Dont,
}

#[derive(Debug)]
#[parsable(find_many_skipping_unknown())]
struct Day {
    commands: Vec<Command>,
}

impl ExecutableDay for Day {
    type Output = i64;

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
