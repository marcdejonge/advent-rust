#![feature(test)]

use advent_lib::day_main;
use advent_macros::parsable;
use std::cmp::max;

#[parsable(separated_lines1())]
struct Input {
    games: Vec<Game>,
}

#[parsable(separated_pair(
    preceded(tag(b"Game "), u64),
    tag(b": "),
    separated_list1(tag(b"; "), Draw::parser())
))]
#[derive(Debug)]
struct Game {
    index: u64,
    draws: Vec<Draw>,
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

impl advent_lib::parsing::Parsable for Draw {
    fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        use nom::branch::alt;
        use nom::bytes::complete::tag;
        use nom::character::complete::u64;
        use nom::combinator::map;
        use nom::multi::separated_list1;
        use nom::sequence::terminated;

        map(
            separated_list1(
                tag(b", "),
                alt((
                    map(terminated(u64, tag(b" red")), Draw::red),
                    map(terminated(u64, tag(b" green")), Draw::green),
                    map(terminated(u64, tag(b" blue")), Draw::blue),
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

fn calculate_part1(input: &Input) -> u64 {
    input
        .games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.index)
        .sum()
}

fn calculate_part2(input: &Input) -> u64 {
    input
        .games
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

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example => 8, 2286 );
    day_test!( 2 => 2716, 72227 );
}
