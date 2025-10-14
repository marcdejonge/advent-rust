#![feature(test)]

use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::geometry::{vector2, Vector};
use advent_lib::grid::{Grid, Location};
use advent_lib::*;
use advent_macros::FromRepr;
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use std::iter::successors;
use Block::*;

#[parse_from(Grid::parse)]
struct Input {
    grid: Grid<Block>,
    #[derived({
        let start = grid.find(|&b| b == Start).unwrap();
        walk_grid(&grid, start).enumerate().collect()
    })]
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

impl Input {
    fn find_cheats(&self, locations: &[Vector<2, i32>], min_saved: usize) -> usize {
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
                                    && (end_dist - (start_dist + cheat_dist)) >= min_saved
                            })
                            .unwrap_or(false)
                    })
                    .count()
            })
            .sum()
    }
}

fn calculate_part1(input: &Input) -> usize { input.find_cheats(&generate_steps(2), 100) }
fn calculate_part2(input: &Input) -> usize { input.find_cheats(&generate_steps(20), 100) }

day_main!();
day_test!( 20, example1 => 0, 0 );
day_test!( 20 => 1459, 1016066 );
