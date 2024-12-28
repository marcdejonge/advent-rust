#![feature(test)]

use advent_lib::day::*;
use advent_lib::geometry::{vector2, Vector};
use advent_lib::grid::{Grid, Location};
use advent_lib::parsing::map_parser;
use nom::error::Error;
use nom::Parser;

struct Day {
    grid: Grid<u8>,
}

type Step = Vector<2, i32>;

impl Day {
    fn check_ms_around_a(&self, location: Location, first: Step, second: Step) -> bool {
        match self.grid.get(location + first) {
            Some(&b'M') => self.grid.get(location + second) == Some(&b'S'),
            Some(&b'S') => self.grid.get(location + second) == Some(&b'M'),
            _ => false,
        }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map_parser(|grid| Day { grid })
    }

    fn calculate_part1(&self) -> Self::Output {
        const DIRECTIONS: [Step; 8] = [
            vector2(1, -1),
            vector2(1, 0),
            vector2(1, 1),
            vector2(0, -1),
            vector2(0, 1),
            vector2(-1, -1),
            vector2(-1, 0),
            vector2(-1, 1),
        ];

        self.grid
            .entries()
            .filter(|(_, &char)| char == b'X')
            .map(|(location, _)| {
                DIRECTIONS
                    .iter()
                    .filter(|&&dir| {
                        self.grid.get(location + dir) == Some(&b'M')
                            && self.grid.get(location + dir * 2) == Some(&b'A')
                            && self.grid.get(location + dir * 3) == Some(&b'S')
                    })
                    .count()
            })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.grid
            .entries()
            .filter(|(location, &char)| {
                char == b'A'
                    && self.check_ms_around_a(*location, vector2(1, -1), vector2(-1, 1))
                    && self.check_ms_around_a(*location, vector2(1, 1), vector2(-1, -1))
            })
            .count()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example1 => 18, 9 );
    day_test!( 4 => 2530, 1921);
}
