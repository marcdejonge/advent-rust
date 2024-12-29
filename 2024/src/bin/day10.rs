#![feature(test)]

use advent_lib::day_main;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::grid::Location;
use fxhash::FxHashSet;
use rayon::prelude::*;

type Grid = advent_lib::grid::Grid<u8>;

fn find_unique_trail_locations(grid: &Grid, location: Location, endings: &mut FxHashSet<Location>) {
    match grid.get(location) {
        None => {}
        Some(&b'9') => {
            endings.insert(location);
        }
        Some(&current) => {
            neighbours(grid, location, current + 1)
                .for_each(|loc| find_unique_trail_locations(grid, loc, endings));
        }
    }
}

fn find_all_trails(grid: &Grid, location: Location) -> u32 {
    match grid.get(location) {
        None => 0,
        Some(&b'9') => 1,
        Some(&current) => neighbours(grid, location, current + 1)
            .map(|loc| find_all_trails(grid, loc))
            .sum(),
    }
}

fn neighbours(grid: &Grid, loc: Location, next: u8) -> impl Iterator<Item = Location> + use<'_> {
    ALL_DIRECTIONS
        .iter()
        .map(move |d| loc + d.as_vec())
        .filter(move |&loc| grid.get(loc) == Some(&next))
}

fn start_nodes(grid: &Grid) -> impl Iterator<Item = Location> + use<'_> {
    grid.entries().filter(|(_, &c)| c == b'0').map(|(loc, _)| loc)
}

fn calculate_part1(grid: &Grid) -> usize {
    start_nodes(grid)
        .par_bridge()
        .map(|loc| {
            let mut result = Default::default();
            find_unique_trail_locations(grid, loc, &mut result);
            result.len()
        })
        .sum()
}

fn calculate_part2(grid: &Grid) -> u32 {
    start_nodes(grid).par_bridge().map(|loc| find_all_trails(grid, loc)).sum()
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 10, example1 => 36, 81 );
    day_test!( 10 => 737, 1619);
}
