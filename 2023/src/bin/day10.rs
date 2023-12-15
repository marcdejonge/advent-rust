/*
--- Day 10: Pipe Maze ---

You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal
island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so
you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do
occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction;
maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire
some metal grass, you notice something metallic scurry away in your peripheral vision and jump into
a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need
to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely packed with
pipes; it was hard to tell at first because they're the same metallic silver color as the "ground".
You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't
show what shape the pipe has.
Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal
is one large, continuous loop.

Find the single giant loop starting at S. How many steps along the loop does it take to get from the
starting position to the point farthest from the starting position?

--- Part Two ---

You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is
within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate
how many tiles are contained within the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop.
How many tiles are enclosed by the loop?
*/
#![feature(test)]

extern crate core;

use std::ops::{Add, Neg};

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction::*;
use advent_lib::direction::{Direction, ALL_DIRECTIONS};
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_macros::FromRepr;

use crate::PipeCell::*;

struct Day {
    grid: Grid<PipeCell>,
    start: Point<2, i32>,
}

impl Day {
    fn iter(&self) -> DayWalker {
        DayWalker {
            day: self,
            started: false,
            location: self.start,
            direction: *ALL_DIRECTIONS
                .iter()
                .find(|&&d| self.grid[self.start].points_to(d))
                .unwrap(),
        }
    }
}

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Eq, PartialEq, Debug)]
enum PipeCell {
    #[display = ' ']
    Ground = b'.',
    Start = b'S',
    #[display('┗')]
    NorthEast = b'L',
    #[display('┃')]
    NorthSouth = b'|',
    #[display('┛')]
    NorthWest = b'J',
    #[display('┏')]
    EastSouth = b'F',
    #[display('━')]
    EastWest = b'-',
    #[display('┓')]
    SouthWest = b'7',
}

impl PipeCell {
    fn get_next_direction(&self, from: Direction) -> Option<Direction> {
        match (self, from) {
            (NorthEast, South) => Some(East),
            (NorthEast, West) => Some(North),
            (NorthSouth, South) => Some(South),
            (NorthSouth, North) => Some(North),
            (NorthWest, South) => Some(West),
            (NorthWest, East) => Some(North),
            (EastSouth, West) => Some(South),
            (EastSouth, North) => Some(East),
            (EastWest, West) => Some(West),
            (EastWest, East) => Some(East),
            (SouthWest, North) => Some(West),
            (SouthWest, East) => Some(South),
            _ => None,
        }
    }

    fn points_to(&self, to: Direction) -> bool {
        matches!(
            (self, to),
            (NorthEast, North)
                | (NorthEast, East)
                | (NorthSouth, North)
                | (NorthSouth, South)
                | (NorthWest, North)
                | (NorthWest, West)
                | (EastSouth, East)
                | (EastSouth, South)
                | (EastWest, East)
                | (EastWest, West)
                | (SouthWest, South)
                | (SouthWest, West)
        )
    }

    fn detect_pipe(grid: &Grid<PipeCell>, location: Point<2, i32>) -> Option<PipeCell> {
        let connected = ALL_DIRECTIONS
            .map(|d| grid.get(location.add(d.as_vec())).unwrap_or(&Ground).points_to(d.neg()));

        match connected {
            [true, true, false, false] => Some(NorthEast),
            [true, false, true, false] => Some(NorthSouth),
            [true, false, false, true] => Some(NorthWest),
            [false, true, true, false] => Some(EastSouth),
            [false, true, false, true] => Some(EastWest),
            [false, false, true, true] => Some(SouthWest),
            _ => None,
        }
    }
}

struct DayWalker<'a> {
    day: &'a Day,
    started: bool,
    location: Point<2, i32>,
    direction: Direction,
}

impl<'a> Iterator for DayWalker<'a> {
    type Item = (Point<2, i32>, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
        } else {
            self.location = self.location.add(self.direction.as_vec());
            if self.location == self.day.start {
                return None;
            }
            self.direction =
                self.day.grid.get(self.location)?.get_next_direction(self.direction)?;
        }

        Some((self.location, self.direction))
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut grid = Grid::from(lines);
        let start = grid.find(|item| item == &Start).unwrap();
        let start_pipe = PipeCell::detect_pipe(&grid, start).unwrap();

        if let Some(cell) = grid.get_mut(start) {
            *cell = start_pipe
        }

        Day { start, grid }
    }

    fn calculate_part1(&self) -> Self::Output { self.iter().count() / 2 }

    fn calculate_part2(&self) -> Self::Output {
        #[derive(Copy, Clone, Eq, PartialEq, Default)]
        enum LocationType {
            #[default]
            Background,
            Inside,
            VerticalPipe,
            HorizontalPipe,
        }

        let mut pipe_grid =
            Grid::<LocationType>::new_empty(self.grid.x_range(), self.grid.y_range());
        self.iter().for_each(|(loc, _)| {
            if let Some(cell) = pipe_grid.get_mut(loc) {
                let pipe = self.grid[loc];
                *cell = if pipe.points_to(North) {
                    LocationType::VerticalPipe
                } else {
                    LocationType::HorizontalPipe
                }
            }
        });

        for y in pipe_grid.y_range() {
            let mut outside = true;
            for x in pipe_grid.x_range() {
                let cell = pipe_grid.get_mut(point2(x, y)).unwrap();
                match *cell {
                    LocationType::Background => {
                        if !outside {
                            *cell = LocationType::Inside;
                        }
                    }
                    LocationType::VerticalPipe => {
                        outside = !outside;
                    }
                    _ => {}
                }
            }
        }

        pipe_grid.values().filter(|t| **t == LocationType::Inside).count()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 10, example1 => 4, 1 );
    day_test!( 10, example2 => 8, 1 );
    day_test!( 10, example3 => 23, 4 );
    day_test!( 10, example4 => 22, 4 );
    day_test!( 10, example5 => 70, 8 );
    day_test!( 10 => 6714, 429 );
}
