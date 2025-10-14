#![feature(test)]

use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::grid::{Grid, Location};
use advent_lib::*;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[parse_from]
enum Field {
    #[format(".")]
    Empty,
    #[format("#")]
    Blocked,
    #[format(Direction::parse)]
    Visited(Direction),
}

fn guard_walk(grid: &mut Grid<Field>, start: &Location) -> bool {
    let mut guard_pos = *start;
    let mut guard_dir = North;
    loop {
        let next = guard_pos + guard_dir.as_vec();
        match grid.get(next) {
            None => return true,
            Some(Field::Visited(dir)) if dir == &guard_dir => return false,
            Some(Field::Blocked) => guard_dir = guard_dir.turn_right(),
            _ => {
                grid[next] = Field::Visited(guard_dir);
                guard_pos = next;
            }
        }
    }
}

fn get_start(grid: &Grid<Field>) -> Location { grid.find(|f| f == &Field::Visited(North)).unwrap() }

fn calculate_part1(grid: &Grid<Field>) -> usize {
    let start = get_start(grid);
    let mut grid = grid.clone();
    guard_walk(&mut grid, &start);
    grid.values().filter(|&f| matches!(f, &Field::Visited(_))).count()
}

fn calculate_part2(grid: &Grid<Field>) -> usize {
    let start = get_start(grid);
    grid.entries()
        .par_bridge()
        .filter(|(_, &f)| f != Field::Blocked)
        .filter(|(loc, _)| {
            let mut grid = grid.clone();
            grid[*loc] = Field::Blocked;
            !guard_walk(&mut grid, &start)
        })
        .count()
}

day_main!();
day_test!( 6, example1 => 41, 6 );
day_test!( 6 => 4711, 1562);
