#![feature(test)]

use crate::Block::*;
use advent_lib::geometry::{Vector, vector2};
use advent_lib::grid::{Grid, Location};
use advent_lib::*;
use nom_parse_macros::parse_from;

type Step = Vector<2, i32>;

#[parse_from]
#[derive(Clone, Copy, PartialEq)]
enum Block {
    #[format("X")]
    X,
    #[format("M")]
    M,
    #[format("A")]
    A,
    #[format("S")]
    S,
}

fn check_ms_around_a(grid: &Grid<Block>, location: Location, first: Step, second: Step) -> bool {
    match grid.get(location + first) {
        Some(&M) => grid.get(location + second) == Some(&S),
        Some(&S) => grid.get(location + second) == Some(&M),
        _ => false,
    }
}

fn calculate_part1(grid: &Grid<Block>) -> usize {
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

    grid.entries()
        .filter(|&(_, char)| *char == X)
        .map(|(location, _)| {
            DIRECTIONS
                .iter()
                .filter(|&&dir| {
                    grid.get(location + dir) == Some(&M)
                        && grid.get(location + dir * 2) == Some(&A)
                        && grid.get(location + dir * 3) == Some(&S)
                })
                .count()
        })
        .sum()
}

fn calculate_part2(grid: &Grid<Block>) -> usize {
    grid.entries()
        .filter(|&(location, char)| {
            *char == A
                && check_ms_around_a(grid, location, vector2(1, -1), vector2(-1, 1))
                && check_ms_around_a(grid, location, vector2(1, 1), vector2(-1, -1))
        })
        .count()
}

day_main!(Grid<Block>);
day_test!( 4, example1 => 18, 9 );
day_test!( 4 => 2530, 1921);
