#![feature(test)]

use advent_lib::day_main;
use advent_lib::direction::Direction;
use advent_lib::geometry::point2;
use advent_lib::grid::Grid;
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::lines::LineSegment;
use advent_macros::FromRepr;
use nom_parse_macros::parse_from;
use std::ops::Add;

type Line = LineSegment<2, i32>;
type Point = advent_lib::geometry::Point<2, i32>;

#[parse_from(map(
    separated_list1(line_ending,
        map(
            separated_list1(" -> ", Point::parse),
            |points| points.into_iter().zip_with_next().map(|(start, end)|
                LineSegment { start, end }
            )
        )
    ),
    |lines| lines.into_iter().flatten().collect(),
))]
struct Lines(Vec<Line>);

fn generate_grid(lines: Lines) -> Grid<Place> {
    let max_height = lines.0.iter().map(|line| line.max_y()).max().unwrap() + 2;
    let mut grid = Grid::new_empty(1000, max_height + 1);
    for line in lines.0 {
        draw_line(&mut grid, line, Place::Line);
    }
    grid
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

fn calculate_part1(grid: &Grid<Place>) -> usize { SandDroppingGrid::new(&grid).count() }

fn calculate_part2(grid: &Grid<Place>) -> usize {
    let mut grid = SandDroppingGrid::new(&grid);
    let y = grid.grid.height() - 1;
    for x in grid.grid.x_range() {
        let place = grid.grid.get_mut(point2(x, y)).unwrap();
        *place = Place::Line;
    }
    grid.count()
}

struct SandDroppingGrid {
    grid: Grid<Place>,
    drop_point: Point,
}

impl SandDroppingGrid {
    fn new(grid: &Grid<Place>) -> SandDroppingGrid {
        SandDroppingGrid { grid: grid.clone(), drop_point: point2(500, 0) }
    }
}
impl Iterator for SandDroppingGrid {
    type Item = Point;

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

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Place {
    #[default]
    Empty = b'.',
    Sand = b'o',
    Line = b'#',
}

day_main!( generate_grid => calculate_part1, calculate_part2 );

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 14, example => 24, 93 ; generate_grid );
    day_test!( 14 => 843, 27625 ; generate_grid );
}
