#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::grid::{Grid, Location};
use advent_lib::parsing::map_parser;
use fxhash::FxHashSet;
use nom::error::Error;
use nom::Parser;
use rayon::prelude::*;

struct Day {
    grid: Grid<u8>,
}

impl Day {
    fn find_unique_trail_locations(&self, location: Location, endings: &mut FxHashSet<Location>) {
        match self.grid.get(location) {
            None => {}
            Some(&b'9') => {
                endings.insert(location);
            }
            Some(&current) => {
                self.neighbours(location, current + 1)
                    .for_each(|loc| self.find_unique_trail_locations(loc, endings));
            }
        }
    }

    fn find_all_trails(&self, location: Location) -> u32 {
        match self.grid.get(location) {
            None => 0,
            Some(&b'9') => 1,
            Some(&current) => self
                .neighbours(location, current + 1)
                .map(|loc| self.find_all_trails(loc))
                .sum(),
        }
    }

    fn neighbours(&self, loc: Location, next: u8) -> impl Iterator<Item = Location> + use<'_> {
        ALL_DIRECTIONS
            .iter()
            .map(move |d| loc + d.as_vec())
            .filter(move |&loc| self.grid.get(loc) == Some(&next))
    }

    fn start_nodes(&self) -> impl Iterator<Item = Location> + use<'_> {
        self.grid.entries().filter(|(_, &c)| c == b'0').map(|(loc, _)| loc)
    }
}

impl ExecutableDay for Day {
    type Output = u32;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map_parser(|grid| Day { grid })
    }

    fn calculate_part1(&self) -> Self::Output {
        self.start_nodes()
            .par_bridge()
            .map(|loc| {
                let mut result = Default::default();
                self.find_unique_trail_locations(loc, &mut result);
                result.len() as u32
            })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.start_nodes().par_bridge().map(|loc| self.find_all_trails(loc)).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 10, example1 => 36, 81 );
    day_test!( 10 => 737, 1619);
}
