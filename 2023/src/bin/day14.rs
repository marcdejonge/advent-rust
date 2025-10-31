#![feature(test)]

use std::ops::Neg;

use fxhash::hash64;

use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::*;
use advent_macros::FromRepr;

#[repr(u8)]
#[derive(FromRepr, PartialEq, Clone, Hash)]
enum Stone {
    None = b'.',
    Fixed = b'#',
    Rolling = b'O',
}

fn drop_cell(
    grid: &mut Grid<Stone>,
    last_location: &mut Point<2, i32>,
    location: Point<2, i32>,
    direction: Direction,
) {
    match grid[location] {
        Stone::Fixed => {
            *last_location = location + direction.as_vec();
        }
        Stone::Rolling => {
            grid.swap(location, *last_location);
            *last_location += direction.as_vec()
        }
        Stone::None => {}
    }
}

fn drop(grid: &mut Grid<Stone>, direction: Direction) {
    let direction = direction.neg();
    match direction {
        North => {
            grid.x_range().for_each(|x| {
                let mut last_location = point2(x, grid.height() - 1);
                for y in grid.y_range().rev() {
                    drop_cell(grid, &mut last_location, point2(x, y), North)
                }
            });
        }
        West => {
            grid.y_range().for_each(|y| {
                let mut last_location = point2(grid.width() - 1, y);
                for x in grid.x_range().rev() {
                    drop_cell(grid, &mut last_location, point2(x, y), West)
                }
            });
        }
        South => {
            grid.x_range().for_each(|x| {
                let mut last_location = point2(x, 0);
                for y in grid.y_range() {
                    drop_cell(grid, &mut last_location, point2(x, y), South)
                }
            });
        }
        East => {
            grid.y_range().for_each(|y| {
                let mut last_location = point2(0, y);
                for x in grid.x_range() {
                    drop_cell(grid, &mut last_location, point2(x, y), East)
                }
            });
        }
    }
}

fn weight(grid: &Grid<Stone>) -> i32 {
    grid.entries()
        .filter(|(_, stone)| **stone == Stone::Rolling)
        .map(|(location, _)| grid.height() - location.coords[1])
        .sum()
}

fn calculate_part1(grid: &Grid<Stone>) -> i32 {
    let mut grid = grid.clone();

    drop(&mut grid, North);
    weight(&grid)
}

fn calculate_part2(grid: &Grid<Stone>) -> i32 {
    let mut grid = grid.clone();

    (0..)
        .map(|_| {
            drop(&mut grid, North);
            drop(&mut grid, West);
            drop(&mut grid, South);
            drop(&mut grid, East);

            (hash64(&grid), weight(&grid))
        })
        .find_cyclic_result_at(1000000000)
        .unwrap()
}

day_main!(Grid<Stone>);
day_test!( 14, example => 136, 64);
day_test!( 14 => 110407, 87273);
