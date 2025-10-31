#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;
use std::cmp::max;

#[parse_from(separated_pair(preceded("Game ", u64), ": ", separated_list1("; ", Draw::parse)))]
#[derive(Debug)]
struct Game {
    index: u64,
    draws: Vec<Draw>,
}
#[parse_from(
    map(separated_list1(
        tag(", "),
        alt((
                map(terminated(u64, tag(" red")), Draw::red),
                map(terminated(u64, tag(" green")), Draw::green),
                map(terminated(u64, tag(" blue")), Draw::blue),
            )),
    ), |draws| {
        draws.iter().fold((0,0,0), |(cr, cg, cb), next| (
            cr + next.red,
            cg + next.green,
            cb + next.blue,
        ))
    })
)]
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

fn calculate_part1(games: &[Game]) -> u64 {
    games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.index)
        .sum()
}

fn calculate_part2(games: &[Game]) -> u64 {
    games
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

day_main!(Vec<Game>);
day_test!( 2, example => 8, 2286 );
day_test!( 2 => 2716, 72227 );
