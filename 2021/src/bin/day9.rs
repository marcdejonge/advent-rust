#![feature(test)]

use advent_lib::{
    grid::{Grid, Location},
    iter_utils::{IteratorUtils, SumWith},
    parsing::single_digit,
    *,
};
use nom_parse_macros::parse_from;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[parse_from(map(single_digit(), |d| d - b'0'))]
struct Height(u8);

fn find_low_points(grid: &Grid<Height>) -> impl Iterator<Item = (Location, &Height)> {
    grid.entries().filter(|&(loc, height)| {
        grid.direct_neighbours(loc)
            .all(|(_, neighbour)| neighbour > height)
    })
}

fn calculate_part1(grid: &Grid<Height>) -> u64 {
    find_low_points(grid).sum_with(|(_, height)| height.0 as u64 + 1)
}

fn calculate_part2(grid: &Grid<Height>) -> usize {
    let mut fill = grid.map(|height| if height.0 == 9 { b'X' } else { 0 });
    let biggest: [usize; 3] = find_low_points(grid)
        .map(|(loc, _)| fill.fill(loc, 1 - fill.get(loc).unwrap()))
        .max_n();
    biggest.iter().product()
}

day_main!();

day_test!( 9, example => 15, 1134 );
day_test!( 9 => 532, 1110780 );
