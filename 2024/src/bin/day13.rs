#![feature(test)]

use advent_lib::day::*;
use advent_lib::geometry::{vector2, Vector};
use advent_macros::parsable;

type Move = Vector<2, i64>;

#[parsable(separated_double_lines1())]
struct Day {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
#[parsable(tuple((
    map(
        delimited(tag(b"Button A: X+"), separated_pair(i64, tag(b", Y+"), i64), line_ending),
        |(x, y)| vector2(x, y),
    ),
    map(
        delimited(tag(b"Button B: X+"), separated_pair(i64, tag(b", Y+"), i64), line_ending),
        |(x, y)| vector2(x, y),
    ),
    map(
        preceded(tag(b"Prize: X="), separated_pair(i64, tag(b", Y="), i64)),
        |(x, y)| vector2(x, y),
    ),
)))]
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
