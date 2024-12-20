#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::geometry::{vector2, Vector};
use advent_lib::grid::{Grid, Location};
use advent_macros::FromRepr;
use rayon::prelude::*;
use std::iter::successors;
use Block::*;

struct Day {
    grid: Grid<Block>,
    walk: Vec<(usize, Location)>,
}

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, PartialEq)]
enum Block {
    Empty = b'.',
    Wall = b'#',
    Start = b'S',
    End = b'E',
}

fn walk_grid(grid: &Grid<Block>, start: Location) -> impl Iterator<Item = Location> + '_ {
    let start_dir = *ALL_DIRECTIONS.iter().find(|&&d| grid[start + d] != Wall).unwrap();
    successors(Some((start, start_dir)), move |&(last_loc, last_dir)| {
        let next_dir = *[last_dir, last_dir.turn_left(), last_dir.turn_right()]
            .iter()
            .find(|&&dir| grid[last_loc + dir] != Wall)?;

        Some((last_loc + next_dir, next_dir))
    })
    .map(|(loc, _)| loc)
}

fn generate_steps(max_size: i32) -> Vec<Vector<2, i32>> {
    (-max_size..=max_size)
        .flat_map(|x| (-max_size..=max_size).map(move |y| vector2(x, y)))
        .filter(|&v| (2..=max_size).contains(&v.euler()))
        .collect()
}

impl Day {
    fn find_shortcuts(&self, locations: &[Vector<2, i32>]) -> usize {
        let mut distances = Grid::new_empty(self.grid.width(), self.grid.height());
        self.walk.iter().for_each(|&(ix, loc)| distances[loc] = ix);

        self.walk
            .par_iter()
            .map(|&(start_dist, start_loc)| {
                locations
                    .iter()
                    .filter(|&&step| {
                        distances
                            .get(start_loc + step)
                            .map(|&end_dist| {
                                let cheat_dist = step.euler() as usize;
                                end_dist > start_dist
                                    && (end_dist - (start_dist + cheat_dist)) >= 100
                            })
                            .unwrap_or(false)
                    })
                    .count()
            })
            .sum()
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let grid = Grid::from(lines);
        let start = grid.find(|&b| b == Start).unwrap();
        let walk = walk_grid(&grid, start).enumerate().collect();
        Day { grid, walk }
    }

    fn calculate_part1(&self) -> Self::Output { self.find_shortcuts(&generate_steps(2)) }
    fn calculate_part2(&self) -> Self::Output { self.find_shortcuts(&generate_steps(20)) }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 20, example1 => 0, 0 );
    day_test!( 20 => 1459, 1016066 );
}
