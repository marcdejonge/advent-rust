#![feature(map_try_insert)]
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
#![feature(iter_collect_into)]

extern crate core;

use std::ops::{Add, Neg};

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction::*;
use advent_lib::direction::{Direction, ALL_DIRECTIONS};
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;

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
            direction: if let Pipe { directions } = self.grid[self.start] {
                directions[0]
            } else {
                panic!("No direction for starting location")
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PipeCell {
    Ground,
    Pipe { directions: [Direction; 2] },
}

impl PipeCell {
    fn get_next_direction(&self, from: Direction) -> Option<Direction> {
        if let Pipe { directions: [a, b] } = *self {
            if a == from.neg() {
                Some(b)
            } else if b == from.neg() {
                Some(a)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn detect_pipe(grid: &Grid<PipeCell>, location: Point<2, i32>) -> Option<PipeCell> {
        Some(Pipe {
            directions: ALL_DIRECTIONS
                .into_iter()
                .filter(|d| {
                    let cell = grid.get(location.add(d.as_vec()));
                    if let Some(Pipe { directions }) = cell {
                        directions[0] == d.neg() || directions[1] == d.neg()
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .ok()?,
        })
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

#[derive(Copy, Clone, Eq, PartialEq, Default)]
enum LocationType {
    #[default]
    Background,
    Inside,
    VerticalPipe,
    HorizontalPipe,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut start = Point::default();
        let mut grid = Grid::new(
            lines
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| match c {
                            'S' => {
                                start = point2(x as i32, y as i32);
                                Ground
                            }
                            'J' => Pipe { directions: [North, West] },
                            '|' => Pipe { directions: [North, South] },
                            'L' => Pipe { directions: [North, East] },
                            'F' => Pipe { directions: [East, South] },
                            '-' => Pipe { directions: [East, West] },
                            '7' => Pipe { directions: [South, West] },
                            _ => Ground,
                        })
                        .collect()
                })
                .collect(),
        );
        let start_pipe = PipeCell::detect_pipe(&grid, start).unwrap();

        if let Some(cell) = grid.get_mut(start) {
            *cell = start_pipe
        }

        Day { start, grid }
    }

    fn calculate_part1(&self) -> Self::Output { self.iter().count() / 2 }

    fn calculate_part2(&self) -> Self::Output {
        let mut pipe_grid =
            Grid::<LocationType>::new_empty(self.grid.x_range(), self.grid.y_range());
        self.iter().for_each(|(loc, _)| {
            if let Some(cell) = pipe_grid.get_mut(loc) {
                let pipe = self.grid[loc];
                *cell = if matches!(pipe, Pipe { directions: [North, _] }) {
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
