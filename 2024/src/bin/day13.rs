#![feature(test)]

use advent_lib::day::*;
use advent_lib::geometry::{vector2, Vector};
use advent_lib::parsing::double_line_ending;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::Parser;

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

impl ExecutableDay for Day {
    type Output = i64;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                double_line_ending,
                tuple((
                    delimited(
                        tag(b"Button A: X+"),
                        separated_pair(complete::i64, tag(b", Y+"), complete::i64),
                        line_ending,
                    ),
                    delimited(
                        tag(b"Button B: X+"),
                        separated_pair(complete::i64, tag(b", Y+"), complete::i64),
                        line_ending,
                    ),
                    preceded(
                        tag(b"Prize: X="),
                        separated_pair(complete::i64, tag(b", Y="), complete::i64),
                    ),
                )),
            ),
            |games| Day {
                games: games
                    .into_iter()
                    .map(|(button_a, button_b, target)| Game {
                        button_a: button_a.into(),
                        button_b: button_b.into(),
                        target: target.into(),
                    })
                    .collect(),
            },
        )
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
