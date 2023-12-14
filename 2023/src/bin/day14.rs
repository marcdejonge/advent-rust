/*

*/
#![feature(test)]

use fxhash::hash64;
use std::ops::Neg;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::iter_utils::DetectingCycleTrait;

struct Day {
    grid: Grid<char>,
}

fn drop_cell(
    grid: &mut Grid<char>,
    last_location: &mut Point<2, i32>,
    location: Point<2, i32>,
    direction: Direction,
) {
    match grid[location] {
        '#' => {
            *last_location = location + direction.as_vec();
        }
        'O' => {
            grid.swap(location, *last_location).unwrap();
            *last_location = *last_location + direction.as_vec()
        }
        _ => {}
    }
}

fn drop(grid: &mut Grid<char>, direction: Direction) {
    let direction = direction.neg();
    match direction {
        North => {
            grid.x_range().for_each(|x| {
                let mut last_location = point2(x, *grid.y_range().end());
                for y in grid.y_range().rev() {
                    drop_cell(grid, &mut last_location, point2(x, y), direction)
                }
            });
        }
        West => {
            grid.y_range().for_each(|y| {
                let mut last_location = point2(*grid.x_range().end(), y);
                for x in grid.x_range().rev() {
                    drop_cell(grid, &mut last_location, point2(x, y), direction)
                }
            });
        }
        South => {
            grid.x_range().for_each(|x| {
                let mut last_location = point2(x, *grid.y_range().start());
                for y in grid.y_range() {
                    drop_cell(grid, &mut last_location, point2(x, y), direction)
                }
            });
        }
        East => {
            grid.y_range().for_each(|y| {
                let mut last_location = point2(*grid.x_range().start(), y);
                for x in grid.x_range() {
                    drop_cell(grid, &mut last_location, point2(x, y), direction)
                }
            });
        }
    }
}

fn weight(grid: &Grid<char>) -> usize {
    let mut sum = 0;
    for x in grid.x_range() {
        for y in grid.y_range() {
            if grid[point2(x, y)] == 'O' {
                sum += grid.height() - y as usize;
            }
        }
    }

    sum
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { grid: Grid::parse(lines) }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut grid = self.grid.clone();

        drop(&mut grid, North);
        weight(&grid)
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut grid = self.grid.clone();

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
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 14, example => 136, 64);
    day_test!( 14 => 110407, 87273);
}
