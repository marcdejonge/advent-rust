#![feature(test)]

extern crate core;

use crate::PipeCell::*;
use advent_lib::direction::Direction::*;
use advent_lib::direction::{Direction, ALL_DIRECTIONS};
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::*;
use advent_macros::FromRepr;
use std::ops::{Add, Neg};

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Eq, PartialEq, Debug, Default)]
enum PipeCell {
    #[default]
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

struct GridWalker<'a> {
    grid: &'a Grid<PipeCell>,
    started: bool,
    start: Point<2, i32>,
    location: Point<2, i32>,
    direction: Direction,
}

impl<'a> GridWalker<'a> {
    fn new(grid: &'a Grid<PipeCell>, start: Point<2, i32>) -> GridWalker<'a> {
        GridWalker {
            grid,
            start,
            started: false,
            location: start,
            direction: *ALL_DIRECTIONS.iter().find(|&&d| grid[start].points_to(d)).unwrap(),
        }
    }
}

impl Iterator for GridWalker<'_> {
    type Item = (Point<2, i32>, Direction, PipeCell);

    fn next(&mut self) -> Option<Self::Item> {
        let pipe = if !self.started {
            self.started = true;
            *self.grid.get(self.location)?
        } else {
            self.location = self.location.add(self.direction.as_vec());
            if self.location == self.start {
                return None;
            }
            let pipe = *self.grid.get(self.location)?;
            self.direction = pipe.get_next_direction(self.direction)?;
            pipe
        };

        Some((self.location, self.direction, pipe))
    }
}

fn prepare(input: &Grid<PipeCell>) -> Grid<PipeCell> {
    let mut raw_grid = input.clone();
    let start = raw_grid.find(|item| item == &Start).unwrap();
    let start_pipe = PipeCell::detect_pipe(&raw_grid, start).unwrap();
    if let Some(cell) = raw_grid.get_mut(start) {
        *cell = start_pipe
    }

    let mut grid = Grid::new_empty(raw_grid.width(), raw_grid.height());
    GridWalker::new(&raw_grid, start).for_each(|(loc, _, pipe)| grid[loc] = pipe);
    grid
}

fn calculate_part1(grid: &Grid<PipeCell>) -> usize {
    prepare(grid).values().filter(|p| **p != Ground).count() / 2
}

fn calculate_part2(grid: &Grid<PipeCell>) -> usize {
    #[derive(Copy, Clone, Eq, PartialEq, Default)]
    enum LocationType {
        #[default]
        Background,
        Inside,
        VerticalPipe,
        HorizontalPipe,
    }

    let mut pipe_grid = prepare(grid).map(|pipe| {
        if pipe == &Ground {
            LocationType::Background
        } else if pipe.points_to(North) {
            LocationType::VerticalPipe
        } else {
            LocationType::HorizontalPipe
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

day_main!();
day_test!( 10, example1 => 4, 1 );
day_test!( 10, example2 => 8, 1 );
day_test!( 10, example3 => 23, 4 );
day_test!( 10, example4 => 22, 4 );
day_test!( 10, example5 => 70, 8 );
day_test!( 10 => 6714, 429 );
