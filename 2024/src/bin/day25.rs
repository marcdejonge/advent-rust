#![feature(test)]

use advent_lib::grid::Grid;
use advent_lib::parsing::separated_double_lines1;
use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(separated_double_lines1())]
struct Grids(Vec<Grid<char>>);

#[derive(Debug)]
struct LocksAndKeys {
    locks: Vec<Vec<usize>>,
    keys: Vec<Vec<usize>>,
}

fn parse_grid(grids: Grids) -> LocksAndKeys {
    let mut result = LocksAndKeys { locks: Vec::new(), keys: Vec::new() };
    for grid in grids.0 {
        if grid.east_line(0).all(|(_, &c)| c == '#') {
            result.locks.push(
                grid.x_range()
                    .map(|x| grid.south_line(x).take_while(|(_, &c)| c == '#').count())
                    .collect(),
            )
        } else if grid.east_line(grid.height() - 1).all(|(_, &c)| c == '#') {
            result.keys.push(
                grid.x_range()
                    .map(|x| grid.north_line(x).take_while(|(_, &c)| c == '#').count())
                    .collect(),
            )
        }
    }
    result
}

fn calculate_part1(input: &LocksAndKeys) -> usize {
    input
        .keys
        .iter()
        .flat_map(|key| input.locks.iter().map(move |lock| (key, lock)))
        .filter(|&(key, lock)| (0..key.len()).all(|index| key[index] + lock[index] <= 7))
        .count()
}

day_main!(parse_grid => calculate_part1);
day_test!( 25, example1 => 3 ; crate::parse_grid );
day_test!( 25 => 3021 ; crate::parse_grid );
