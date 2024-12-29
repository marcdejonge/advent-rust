#![feature(test)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

use fxhash::FxHashSet;

use advent_lib::day_main;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::geometry::Point;
use advent_lib::grid::Grid;
use advent_macros::{parsable, FromRepr};

#[parsable(map(Grid::parser(), |mut grid| {
    let start = grid.find(|&space| space == Space::Start).expect("Could not find starting location");
    grid[start] = Space::Ground;
    (grid, start)
}))]
struct Input {
    grid: Grid<Space>,
    start: Point<2, i32>,
}

impl Input {
    fn calculate_far(&self, steps: usize) -> usize {
        let repeat = self.grid.width() as usize; // Assume that the pattern repeats for the size of the grid
        let first = steps % repeat; // The steps are out of sync, so we skip the offset
        if first != self.start.x() as usize && first != self.start.y() as usize {
            panic!("This solution only works if the offset of the start is similar to the mod of the repeat")
        }
        let iterations = steps / repeat;

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
            (iterations * (iterations - 1) / 2 * second_diff) + (iterations * first_diff) + first
        } else {
            0
        }
    }
}

impl<'a> IntoIterator for &'a Input {
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
    day: &'a Input,
    visited_even: FxHashSet<Point<2, i32>>,
    visited_odd: FxHashSet<Point<2, i32>>,
    visited_last_round: Vec<Point<2, i32>>,
    even_round: bool,
    within_bounds: bool,
}

impl ExploreIterator<'_> {
    fn within_bounds(mut self) -> Self {
        self.within_bounds = true;
        self
    }
}

impl Iterator for ExploreIterator<'_> {
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

fn calculate_part1(input: &Input) -> usize { input.into_iter().within_bounds().nth(64).unwrap() }

fn calculate_part2(input: &Input) -> usize { input.calculate_far(26_501_365) }

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 21, example => 42 );
    day_test!( 21 => 3847, 637537341306357 );
}
