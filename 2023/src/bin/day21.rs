#![feature(test)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

use fxhash::FxHashSet;

use advent_lib::day::*;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::geometry::Point;
use advent_lib::grid::Grid;
use advent_macros::FromRepr;

struct Day {
    grid: Grid<Space>,
    start: Point<2, i32>,
}

impl<'a> IntoIterator for &'a Day {
    type Item = usize;
    type IntoIter = ExploreIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ExploreIterator {
            day: self,
            visited_odd: Default::default(),
            visited_even: Default::default(),
            visited_last_round: Default::default(),
            even_round: false,
            within_bounds: false,
        }
    }
}

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, PartialEq)]
enum Space {
    Ground = b'.',
    Rock = b'#',
    Start = b'S',
}

struct ExploreIterator<'a> {
    day: &'a Day,
    visited_even: FxHashSet<Point<2, i32>>,
    visited_odd: FxHashSet<Point<2, i32>>,
    visited_last_round: Vec<Point<2, i32>>,
    even_round: bool,
    within_bounds: bool,
}

impl<'a> ExploreIterator<'a> {
    fn within_bounds(mut self) -> Self {
        self.within_bounds = true;
        self
    }
}

impl<'a> Iterator for ExploreIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.even_round = !self.even_round;
        let visited = if self.even_round { &mut self.visited_even } else { &mut self.visited_odd };

        if self.even_round && visited.is_empty() {
            visited.insert(self.day.start);
            self.visited_last_round = vec![self.day.start];
            return Some(1);
        }

        let mut next_round = Vec::with_capacity(self.visited_last_round.len() * 2);
        while let Some(loc) = self.visited_last_round.pop() {
            ALL_DIRECTIONS
                .into_iter()
                .map(|dir| loc + dir.as_vec())
                .filter(|new_loc| {
                    (if self.within_bounds {
                        self.day.grid.get(*new_loc) == Some(&Space::Ground)
                    } else {
                        self.day.grid.get_infinite(*new_loc) == &Space::Ground
                    }) && visited.insert(*new_loc)
                })
                .for_each(|new_loc| next_round.push(new_loc));
        }

        self.visited_last_round = next_round;

        // self.day.grid.draw_with_overlay(visited.iter(), 'O');

        Some(visited.len())
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut grid = Grid::from(lines);
        let start = grid
            .find(|&space| space == Space::Start)
            .expect("Could not find starting location");
        grid[start] = Space::Ground;

        Day { grid, start }
    }

    fn calculate_part1(&self) -> Self::Output { self.into_iter().within_bounds().nth(64).unwrap() }

    fn calculate_part2(&self) -> Self::Output {
        const STEPS: usize = 26_501_365;
        let repeat = self.grid.width() as usize; // Assume that the pattern repeats for the size of the grid
        let first = STEPS % repeat; // The steps are out of sync, so we skip the offset

        if let &[first, second, third] = self
            .into_iter()
            .enumerate()
            .filter(|(round, _)| round % repeat == first)
            .map(|(_, count)| count)
            .take(3)
            .collect::<Vec<usize>>()
            .as_slice()
        {
            let first_diff = second - first;
            let second_diff = (third - second) - first_diff; // The second order diff is constant
            let n = STEPS / repeat;
            (n * (n - 1) / 2 * second_diff) + (n * first_diff) + first
        } else {
            0
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 21, example => 42 );
    day_test!( 21 => 3847, 637537341306357 );
}
