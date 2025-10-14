#![feature(test)]

use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::grid::Location;
use advent_lib::parsing::single_digit;
use advent_lib::*;
use fxhash::FxHashSet;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[parse_from(single_digit())]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Height(u8);

const BOTTOM: Height = Height(b'0');
const TOP: Height = Height(b'9');

type Grid = advent_lib::grid::Grid<Height>;

impl Height {
    fn next(&self) -> Height { Height(self.0 + 1) }
}

fn find_unique_trail_locations(grid: &Grid, location: Location, endings: &mut FxHashSet<Location>) {
    match grid.get(location) {
        None => {}
        Some(&TOP) => {
            endings.insert(location);
        }
        Some(&current) => {
            neighbours(grid, location, current.next())
                .for_each(|loc| find_unique_trail_locations(grid, loc, endings));
        }
    }
}

fn find_all_trails(grid: &Grid, location: Location) -> u32 {
    match grid.get(location) {
        None => 0,
        Some(&TOP) => 1,
        Some(&current) => neighbours(grid, location, current.next())
            .map(|loc| find_all_trails(grid, loc))
            .sum(),
    }
}

fn neighbours(
    grid: &Grid,
    loc: Location,
    next: Height,
) -> impl Iterator<Item = Location> + use<'_> {
    ALL_DIRECTIONS
        .iter()
        .map(move |d| loc + d.as_vec())
        .filter(move |&loc| grid.get(loc) == Some(&next))
}

fn start_nodes(grid: &Grid) -> impl Iterator<Item = Location> + use<'_> {
    grid.entries().filter(|(_, &c)| c == BOTTOM).map(|(loc, _)| loc)
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
day_test!( 10, example1 => 36, 81 );
day_test!( 10 => 737, 1619);
