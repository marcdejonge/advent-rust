#![feature(test)]
#![feature(iter_collect_into)]

use fxhash::FxHashSet;
use rayon::prelude::*;

use advent_lib::day::*;
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::search::depth_first_search;
use advent_macros::FromRepr;

struct Day {
    grid: Grid<Mirror>,
}

impl Day {
    fn neighbours(
        &self,
        curr_loc: Point<2, i32>,
        curr_dir: Direction,
    ) -> Vec<(Point<2, i32>, Direction)> {
        let mirror = self.grid[curr_loc];

        let mut neighbours = Vec::with_capacity(2);
        let mut add = |next_dir: Direction| {
            let next_loc = curr_loc + next_dir.as_vec();
            if self.grid.is_valid_location(&next_loc) {
                neighbours.push((next_loc, next_dir));
            }
        };

        match (mirror, curr_dir) {
            (Mirror::None, _)
            | (Mirror::SplitHor, East)
            | (Mirror::SplitHor, West)
            | (Mirror::SplitVer, North)
            | (Mirror::SplitVer, South) => add(curr_dir),
            (Mirror::TurnLeft, East) => add(North),
            (Mirror::TurnLeft, North) => add(East),
            (Mirror::TurnLeft, West) => add(South),
            (Mirror::TurnLeft, South) => add(West),
            (Mirror::TurnRight, East) => add(South),
            (Mirror::TurnRight, South) => add(East),
            (Mirror::TurnRight, West) => add(North),
            (Mirror::TurnRight, North) => add(West),
            (Mirror::SplitHor, _) => {
                add(East);
                add(West);
            }
            (Mirror::SplitVer, _) => {
                add(North);
                add(South);
            }
        }

        neighbours
    }

    fn visit_grid(&self, start_loc: Point<2, i32>, start_dir: Direction) -> usize {
        let mut energized_grid = Grid::new_empty(self.grid.width(), self.grid.height());
        let mut visited = FxHashSet::<(Point<2, i32>, Direction)>::default();
        visited.insert((start_loc, start_dir));

        depth_first_search(
            (start_loc, start_dir),
            |(curr_loc, curr_dir)| self.neighbours(curr_loc, curr_dir),
            |state| visited.insert(state),
        );

        visited
            .into_iter()
            .for_each(|(loc, _)| *energized_grid.get_mut(loc).unwrap() = true);

        energized_grid.values().filter(|b| **b).count()
    }
}

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, PartialEq, Hash)]
enum Mirror {
    None = b'.',
    SplitHor = b'-',
    SplitVer = b'|',
    TurnRight = b'\\',
    TurnLeft = b'/',
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { grid: Grid::from(lines) }
    }

    fn calculate_part1(&self) -> Self::Output { self.visit_grid(point2(0, 0), East) }

    fn calculate_part2(&self) -> Self::Output {
        [
            self.grid
                .x_range()
                .map(|x| (point2(x, self.grid.height() - 1), North))
                .collect::<Vec<_>>(),
            self.grid.y_range().map(|y| (point2(0, y), East)).collect(),
            self.grid.x_range().map(|x| (point2(x, 0), South)).collect(),
            self.grid.y_range().map(|y| (point2(self.grid.width() - 1, y), West)).collect(),
        ]
        .into_iter()
        .flatten()
        .par_bridge()
        .map(|(start, dir)| self.visit_grid(start, dir))
        .max()
        .unwrap()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 16, example => 46, 51 );
    day_test!( 16 => 7482, 7896 );
}
