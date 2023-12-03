/*
--- Day 2: Cube Conundrum ---

You're launched high into the atmosphere! The apex of your trajectory just barely reaches the
surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's
quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be
happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many
visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each
time you play this game, he will hide a secret number of cubes of each color in the bag, and your
goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a
handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few
times per game.

You play several games and record the information from each game (your puzzle input). Each game is
listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of
subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

--- Part Two ---

The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why
the water stopped; however, he can show you how to get to the water source to check it out for
yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the
fewest number of cubes of each color that could have been in the bag to make the game possible?

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied
together. For each game, find the minimum set of cubes that must have been present. What is the sum
of the power of these sets?
*/

#![feature(test)]

use std::cmp::max;

use prse::*;

use advent_lib::day::*;

struct Day {
    games: Vec<Game>,
}

#[derive(Parse, Debug)]
#[prse = "Game {index}: {draws:; :}"]
struct Game {
    index: usize,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn empty() -> Self { Draw { red: 0, green: 0, blue: 0 } }
    fn power(&self) -> usize { self.red * self.green * self.blue }
}

impl Parse<'_> for Draw {
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut draw = Draw::empty();

        for color_count in s.split(", ") {
            let (count, color): (usize, String) = parse!(color_count, "{} {}");

            match color.as_str() {
                "red" => draw.red += count,
                "green" => draw.green += count,
                "blue" => draw.blue += count,
                _ => return Err(ParseError::new(format!("Could not parse color {}", color))),
            }
        }

        Ok(draw)
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { games: lines.map(|line| parse!(line, "{}")).collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.games
            .iter()
            .filter(|game| {
                game.draws
                    .iter()
                    .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
            })
            .map(|game| game.index)
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.games
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
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example => 8, 2286 );
    day_test!( 2 => 2716, 72227 );
}
