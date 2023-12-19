#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::search::{a_star_search, SearchGraph, SearchGraphWithGoal};

struct Day {
    grid: Grid<u8>,
}

struct Target<'a> {
    grid: &'a Grid<u8>,
    min_steps: i32,
    max_steps: i32,
}

impl<'a> Target<'a> {
    fn goal(&self) -> Point<2, i32> { point2(self.grid.width() - 1, self.grid.height() - 1) }

    fn calc_path_cost(&self, path: &[(Point<2, i32>, Direction, i32)]) -> usize {
        path.iter()
            .filter_map(
                |(loc, _, _)| if loc.x() == 0 && loc.y() == 0 { None } else { self.grid.get(*loc) },
            )
            .map(|b| *b as usize)
            .sum()
    }
}

impl<'a> SearchGraph for Target<'a> {
    // Location plus number of straight steps we took
    type Node = (Point<2, i32>, Direction, i32);
    type Score = i32;

    fn neighbours(&self, node: Self::Node) -> Vec<(Self::Node, Self::Score)> {
        let (curr_loc, curr_dir, straight_steps) = node;
        let mut neighbours = Vec::with_capacity(3);

        if straight_steps >= self.min_steps || straight_steps == 0 {
            let left = curr_dir.turn_left();
            let left_loc = curr_loc + left.as_vec();
            let left_cell = self.grid.get(left_loc);
            if let Some(cell) = left_cell {
                neighbours.push(((left_loc, left, 1), *cell as i32));
            }

            let right = curr_dir.turn_right();
            let right_loc = curr_loc + right.as_vec();
            let right_cell = self.grid.get(right_loc);
            if let Some(cell) = right_cell {
                neighbours.push(((right_loc, right, 1), *cell as i32));
            }
        }

        if straight_steps < self.max_steps {
            let straight_loc = curr_loc + curr_dir.as_vec();
            let straight_cell = self.grid.get(straight_loc);
            if let Some(cell) = straight_cell {
                neighbours.push(((straight_loc, curr_dir, straight_steps + 1), *cell as i32));
            }
        }

        neighbours
    }
}

impl<'a> SearchGraphWithGoal for Target<'a> {
    fn is_goal(&self, node: Self::Node) -> bool {
        self.goal() == node.0 && node.2 >= self.min_steps
    }

    fn heuristic(&self, node: Self::Node) -> Self::Score { (self.goal() - node.0).euler() }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut grid = Grid::from(lines);
        grid.entries_mut().for_each(|(_, cell)| *cell -= b'0');
        Day { grid }
    }

    fn calculate_part1(&self) -> Self::Output {
        let target = &Target { grid: &self.grid, min_steps: 1, max_steps: 3 };
        let path = a_star_search(target, (point2(0, 0), East, 0)).unwrap();
        target.calc_path_cost(&path)
    }

    fn calculate_part2(&self) -> Self::Output {
        let target = &Target { grid: &self.grid, min_steps: 4, max_steps: 10 };
        let path = a_star_search(target, (point2(0, 0), East, 0)).unwrap();
        target.calc_path_cost(&path)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 17, example => 102, 94 );
    day_test!( 17 => 866, 1010 );
}
