#![feature(test)]

use advent_lib::day::*;
use advent_lib::geometry::{vector2, Vector};
use advent_lib::parsing::digits;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::{IResult, Parser};

type Move = Vector<2, i64>;

struct Day {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
struct Game {
    button_a: Move,
    button_b: Move,
    target: Move,
}

impl Game {
    fn find_score(&self) -> Option<i64> {
        let b = self.button_a.cross(self.target) / self.button_a.cross(self.button_b);
        let a = (self.target.y() - b * self.button_b.y()) / self.button_a.y();

        if self.button_a * a + self.button_b * b == self.target {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Move> {
    alt((
        preceded(
            tag("Button A: X+"),
            separated_pair(digits, tag(", Y+"), digits),
        ),
        preceded(
            tag("Button B: X+"),
            separated_pair(digits, tag(", Y+"), digits),
        ),
        preceded(
            tag("Prize: X="),
            separated_pair(digits, tag(", Y="), digits),
        ),
    ))
    .map(Into::into)
    .parse(input)
}

impl ExecutableDay for Day {
    type Output = i64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            games: lines
                .chunks(4)
                .into_iter()
                .map(|mut lines| {
                    let button_a = parse_line(&lines.next().unwrap()).unwrap().1;
                    let button_b = parse_line(&lines.next().unwrap()).unwrap().1;
                    let target = parse_line(&lines.next().unwrap()).unwrap().1;
                    Game { button_a, button_b, target }
                })
                .collect(),
        }
    }
    fn calculate_part1(&self) -> Self::Output {
        self.games.iter().filter_map(Game::find_score).sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        self.games
            .iter()
            .map(|game| {
                let mut game = game.clone();
                game.target = game.target + vector2(10000000000000, 10000000000000);
                game
            })
            .filter_map(|game| game.find_score())
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 13, example1 => 480, 875318608908 );
    day_test!( 13 => 31897, 87596249540359);
}
