#![feature(test)]

use advent_lib::grid::Grid;
use advent_lib::parsing::separated_double_lines1;
use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(separated_double_lines1())]
struct Grids(Vec<Grid<char>>);

#[derive(Debug)]
#[parse_from(map({}, parse_grid))]
struct LocksAndKeys {
    locks: Vec<Vec<usize>>,
    keys: Vec<Vec<usize>>,
}

fn parse_grid(grids: Grids) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = vec![];
    let mut keys = vec![];
    for grid in grids.0 {
        if grid.east_line(0).all(|(_, &c)| c == '#') {
            locks.push(
                grid.x_range()
                    .map(|x| grid.south_line(x).take_while(|&(_, c)| *c == '#').count())
                    .collect(),
            )
        } else if grid.east_line(grid.height() - 1).all(|(_, c)| *c == '#') {
            keys.push(
                grid.x_range()
                    .map(|x| grid.north_line(x).take_while(|&(_, c)| *c == '#').count())
                    .collect(),
            )
        }
    }
    (locks, keys)
}

fn calculate_part1(input: &LocksAndKeys) -> usize {
    input
        .keys
        .iter()
        .flat_map(|key| input.locks.iter().map(move |lock| (key, lock)))
        .filter(|&(key, lock)| (0..key.len()).all(|index| key[index] + lock[index] <= 7))
        .count()
}

day_main_half!(LocksAndKeys);
day_test!( 25, example1 => 3 );
day_test!( 25 => 3021 );
