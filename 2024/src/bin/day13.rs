#![feature(test)]

use advent_lib::geometry::{vector2, Vector};
use advent_lib::parsing::separated_double_lines1;
use advent_lib::*;
use nom_parse_macros::parse_from;

type Move = Vector<2, i64>;

#[parse_from(separated_double_lines1())]
struct Input {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
#[parse_from((
    map(
        delimited("Button A: X+", separated_pair(i64, ", Y+", i64), line_ending),
        |(x, y)| vector2(x, y),
    ),
    map(
        delimited("Button B: X+", separated_pair(i64, ", Y+", i64), line_ending),
        |(x, y)| vector2(x, y),
    ),
    map(
        preceded("Prize: X=", separated_pair(i64, ", Y=", i64)),
        |(x, y)| vector2(x, y),
    ),
))]
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

fn calculate_part1(input: &Input) -> i64 { input.games.iter().filter_map(Game::find_score).sum() }

fn calculate_part2(input: &Input) -> i64 {
    input
        .games
        .iter()
        .map(|game| {
            let mut game = game.clone();
            game.target = game.target + vector2(10000000000000, 10000000000000);
            game
        })
        .filter_map(|game| game.find_score())
        .sum()
}

day_main!();
day_test!( 13, example1 => 480, 875318608908 );
day_test!( 13 => 31897, 87596249540359);
