#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_macros::{parsable, FromRepr};
use rayon::prelude::*;
use std::ops::{Add, Sub};

use crate::Space::*;

#[parsable(
    map_parsable(|grid: Grid<Space>| {
        let galaxy_locations = grid
            .entries()
            .filter(|(_, space)| **space == Galaxy)
            .map(|(ix, _)| ix)
            .collect();

        ( grid, galaxy_locations )
    })
)]
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
        let mut grid = Grid::new_default(1, self.grid.width(), self.grid.height());

        for y in self.grid.y_range() {
            if self.grid.east_line(y).all(|(_, x)| *x != Galaxy) {
                grid.mut_line(point2(0, y), East.as_vec(), |value| *value = distance)
            }
        }
        for x in self.grid.x_range() {
            if self.grid.south_line(x).all(|(_, x)| *x != Galaxy) {
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
