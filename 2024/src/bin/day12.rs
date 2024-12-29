#![feature(test)]

use advent_lib::day_main;
use advent_lib::direction::CardinalDirections::*;
use advent_lib::direction::Direction::*;
use advent_lib::grid::{Grid, Location};
use advent_lib::iter_utils::{CountIf, SumWith};
use advent_macros::parsable;

#[parsable]
struct Input {
    plot: Grid<u8>,
    #[defer(plot.detect_regions())]
    regions: Vec<Vec<Location>>,
}

impl Input {
    fn sum_regions(&self, grid: &Grid<usize>) -> usize {
        (&self.regions).sum_with(|reg| reg.len() * reg.sum_with(|&loc| grid[loc]))
    }
}

fn calculate_part1(input: &Input) -> usize {
    let fences_grid = input.plot.map_entries(|location, current| {
        (&[North, East, South, West]).count_if(|&&d| input.plot.get(location + d) != Some(current))
    });

    input.sum_regions(&fences_grid)
}

fn calculate_part2(input: &Input) -> usize {
    let corners_grid = input.plot.map_entries(|location, current| {
        (&[[N, NE, E], [E, SE, S], [S, SW, W], [W, NW, N]]).count_if(|dirs| {
            let [forward, diagonal, side] =
                dirs.map(|dir| input.plot.get(location + dir) == Some(current));
            (forward && !diagonal && side) || (!forward && !side)
        })
    });

    input.sum_regions(&corners_grid)
}

day_main!();

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
