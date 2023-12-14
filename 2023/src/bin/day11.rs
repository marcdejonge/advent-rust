/*
--- Day 11: Cosmic Expansion ---

You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf
within turns out to be a researcher studying cosmic expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this research
project. However, he confirms that the hot springs are the next-closest area likely to have people;
he'll even take you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant image (your
puzzle input). The image includes empty space (.) and galaxies (#).

The researcher is trying to figure out the sum of the lengths of the shortest path between every
pair of galaxies. However, there's a catch: the universe expanded in the time it took the light
from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the result is
that any rows or columns that contain no galaxies should all actually be twice as big.

Expand the universe, then find the length of the shortest path between every pair of galaxies.
What is the sum of these lengths?
*/

#![feature(test)]

use rayon::prelude::*;
use std::ops::{Add, Sub};

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::East;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_macros::FromRepr;
use Direction::South;

use crate::Space::*;

struct Day {
    grid: Grid<Space>,
    galaxy_locations: Vec<Point<2, i32>>,
}

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Eq, PartialEq, Debug)]
enum Space {
    EmptySpace = b'.',
    Galaxy = b'#',
}

impl Day {
    fn create_distance_grid(&self, distance: u64) -> Grid<u64> {
        let mut grid = Grid::new_default(1, self.grid.x_range(), self.grid.y_range());

        for y in self.grid.y_range() {
            if self.grid.iter_line(point2(0, y), East.as_vec()).all(|&x| x != Galaxy) {
                grid.mut_line(point2(0, y), East.as_vec(), |value| *value = distance)
            }
        }
        for x in self.grid.x_range() {
            if self.grid.iter_line(point2(x, 0), South.as_vec()).all(|&x| x != Galaxy) {
                grid.mut_line(point2(x, 0), South.as_vec(), |value| *value = distance)
            }
        }

        grid
    }

    fn determine_galaxy_distance_sum(&self, distances: Grid<u64>) -> u64 {
        (0..self.galaxy_locations.len())
            .into_par_iter()
            .map(|from_ix| {
                let from = self.galaxy_locations.get(from_ix).unwrap();
                ((from_ix + 1)..self.galaxy_locations.len())
                    .into_par_iter()
                    .map(|to_ix| {
                        let to = self.galaxy_locations.get(to_ix).unwrap();
                        let diff = to.sub(*from);

                        let mut step = *from;
                        let mut steps = 0;

                        for ix in 0..diff.coords.len() {
                            if diff.coords[ix] != 0 {
                                let direction = diff.coords[ix] / diff.coords[ix].abs();
                                while step.coords[ix] != to.coords[ix] {
                                    step.coords[ix] += direction;
                                    steps += distances[step];
                                }
                            }
                        }

                        steps
                    })
                    .sum()
            })
            .reduce(|| 0, u64::add)
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let grid = Grid::from(lines);
        let galaxy_locations = grid
            .entries()
            .filter(|(_, space)| **space == Galaxy)
            .map(|(ix, _)| ix)
            .collect();

        Day { grid, galaxy_locations }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.determine_galaxy_distance_sum(self.create_distance_grid(2))
    }

    fn calculate_part2(&self) -> Self::Output {
        self.determine_galaxy_distance_sum(self.create_distance_grid(1000000))
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 11, example => 374, 82000210);
    day_test!( 11 => 10490062, 382979724122 );
}
