#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::grid::{Grid, Location};
use advent_lib::parsing::map_parser;
use nom::error::Error;
use nom::Parser;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Field {
    Empty,
    Blocked,
    Visited(Direction),
}

impl From<Field> for char {
    fn from(field: Field) -> Self {
        match field {
            Field::Empty => '.',
            Field::Blocked => '#',
            Field::Visited(North) => '^',
            Field::Visited(East) => '>',
            Field::Visited(South) => 'v',
            Field::Visited(West) => '<',
        }
    }
}

impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Field::Empty,
            b'#' => Field::Blocked,
            b'^' => Field::Visited(North),
            _ => panic!("Illegal byte representation for a Field: {value}"),
        }
    }
}

struct Day {
    grid: Grid<Field>,
    start: Location,
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

impl ExecutableDay for Day {
    type Output = u32;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map_parser(|grid: Grid<Field>| {
            let start = grid.find(|f| f == &Field::Visited(North)).unwrap();
            Day { grid, start }
        })
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut grid = self.grid.clone();
        guard_walk(&mut grid, &self.start);
        grid.values().filter(|&f| matches!(f, &Field::Visited(_))).count() as u32
    }

    fn calculate_part2(&self) -> Self::Output {
        self.grid
            .entries()
            .par_bridge()
            .filter(|(_, &f)| f != Field::Blocked)
            .filter(|(loc, _)| {
                let mut grid = self.grid.clone();
                grid[*loc] = Field::Blocked;
                !guard_walk(&mut grid, &self.start)
            })
            .count() as u32
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 6, example1 => 41, 6 );
    day_test!( 6 => 4711, 1562);
}
