#![feature(test)]

use prse_derive::parse;
use std::ops::Add;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::iter_utils::ZipWithNextTrait;
use advent_lib::lines::LineSegment;

type Line = LineSegment<2, i32>;

struct Day {
    grid: Grid<Place>,
}

fn draw_line(grid: &mut Grid<Place>, line: Line, value: Place) {
    if line.start.x() == line.end.x() {
        let x = line.start.x();
        for y in line.min_y()..=line.max_y() {
            if let Some(place) = grid.get_mut(point2(x, y)) {
                *place = value
            }
        }
    } else if line.start.y() == line.end.y() {
        let y = line.start.y();
        for x in line.min_x()..=line.max_x() {
            if let Some(place) = grid.get_mut(point2(x, y)) {
                *place = value
            }
        }
    } else {
        unimplemented!("Non-straight lines cannot be drawn to a Grid yet")
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let lines: Vec<Line> = lines
            .flat_map(|line: String| {
                parse!(line, "{: -> :}")
                    .into_iter()
                    .zip_with_next::<Point<2, _>>()
                    .map(|(start, end)| LineSegment { start, end })
            })
            .collect();

        let max_height = lines.iter().map(|line| line.max_y()).max().unwrap() + 2;
        let mut grid = Grid::new_empty(1000, max_height + 1);

        for line in lines {
            draw_line(&mut grid, line, Place::Line);
        }

        Day { grid }
    }

    fn calculate_part1(&self) -> Self::Output { SandDroppingGrid::new(&self.grid).count() }

    fn calculate_part2(&self) -> Self::Output {
        let mut grid = SandDroppingGrid::new(&self.grid);
        let y = grid.grid.height() - 1;
        for x in grid.grid.x_range() {
            let place = grid.grid.get_mut(point2(x, y)).unwrap();
            *place = Place::Line;
        }
        grid.count()
    }
}

struct SandDroppingGrid {
    grid: Grid<Place>,
    drop_point: Point<2, i32>,
}

impl SandDroppingGrid {
    fn new(grid: &Grid<Place>) -> SandDroppingGrid {
        SandDroppingGrid { grid: grid.clone(), drop_point: point2(500, 0) }
    }
}
impl Iterator for SandDroppingGrid {
    type Item = Point<2, i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut loc = self.drop_point;

        if self.grid.get(loc) != Some(&Place::Empty) {
            return None;
        }

        loop {
            let down = loc.add(Direction::South.as_vec());
            match self.grid.get(down) {
                Some(&Place::Empty) => loc = down,
                None => return None,
                _ => {
                    let west = down.add(Direction::West.as_vec());
                    if self.grid.get(west) == Some(&Place::Empty) {
                        loc = west;
                    } else {
                        let east = down.add(Direction::East.as_vec());
                        if self.grid.get(east) == Some(&Place::Empty) {
                            loc = east;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        self.grid.get_mut(loc).map(|place| {
            *place = Place::Sand;
            loc
        })
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Place {
    #[default]
    Empty,
    Sand,
    Line,
}

impl From<Place> for char {
    fn from(value: Place) -> Self {
        match value {
            Place::Empty => '.',
            Place::Sand => 'o',
            Place::Line => '#',
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 14, example => 24, 93 );
    day_test!( 14 => 843, 27625 );
}
