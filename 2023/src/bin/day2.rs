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
