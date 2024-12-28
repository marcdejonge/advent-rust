#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::CardinalDirections::*;
use advent_lib::direction::Direction::*;
use advent_lib::grid::{Grid, Location};
use advent_lib::iter_utils::{CountIf, SumWith};
use advent_lib::parsing::map_parser;
use nom::error::Error;
use nom::Parser;

struct Day {
    plot: Grid<u8>,
    regions: Vec<Vec<Location>>,
}

impl Day {
    fn sum_regions(&self, grid: &Grid<usize>) -> usize {
        (&self.regions).sum_with(|reg| reg.len() * reg.sum_with(|&loc| grid[loc]))
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map_parser(|plot: Grid<u8>| {
            let regions = plot.detect_regions();
            Day { plot, regions }
        })
    }

    fn calculate_part1(&self) -> Self::Output {
        let fences_grid = self.plot.map_entries(|location, current| {
            (&[North, East, South, West])
                .count_if(|&&d| self.plot.get(location + d) != Some(current))
        });

        self.sum_regions(&fences_grid)
    }

    fn calculate_part2(&self) -> Self::Output {
        let corners_grid = self.plot.map_entries(|location, current| {
            (&[[N, NE, E], [E, SE, S], [S, SW, W], [W, NW, N]]).count_if(|dirs| {
                let [forward, diagonal, side] =
                    dirs.map(|dir| self.plot.get(location + dir) == Some(current));
                (forward && !diagonal && side) || (!forward && !side)
            })
        });

        self.sum_regions(&corners_grid)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 12, example1 => 140, 80 );
    day_test!( 12, example2 => 772, 436 );
    day_test!( 12, example3 => 1930, 1206 );
    day_test!( 12, example4 => 692, 236 );
    day_test!( 12, example5 => 1184, 368 );
    day_test!( 12 => 1489582, 914966);
}
