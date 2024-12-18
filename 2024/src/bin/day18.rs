#![feature(test)]

use advent_lib::day::*;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::geometry::point2;
use advent_lib::grid::Location;
use advent_lib::search::{a_star_search, SearchGraph, SearchGraphWithGoal};
use fxhash::FxHashSet;
use std::ops::Range;

struct Day {
    locations: Vec<Location>,
    size: i32,
    start_take: usize,
}

struct Memory {
    blocked: FxHashSet<Location>,
    valid_range: Range<i32>,
    target: Location,
}

impl SearchGraph for Memory {
    type Node = Location;
    type Score = i32;

    fn neighbours(&self, loc: Location) -> Vec<(Location, i32)> {
        let mut neighbours = Vec::new();
        for direction in ALL_DIRECTIONS.iter() {
            let neighbour = loc + *direction;
            if self.valid_range.contains(&neighbour.x())
                && self.valid_range.contains(&neighbour.y())
                && !self.blocked.contains(&neighbour)
            {
                neighbours.push((neighbour, 1));
            }
        }
        neighbours
    }
}

impl SearchGraphWithGoal for Memory {
    fn is_goal(&self, loc: Location) -> bool { loc == self.target }
    fn heuristic(&self, loc: Location) -> Self::Score { (loc - self.target).euler() }
}

impl Day {
    fn create_memory(&self, take: usize) -> Memory {
        let blocked = self.locations.iter().take(take).copied().collect();
        let valid_range = 0..self.size;
        let target = point2(self.size - 1, self.size - 1);
        Memory { blocked, valid_range, target }
    }

    fn find_path(&self, take: usize) -> Option<usize> {
        let memory = self.create_memory(take);
        a_star_search(&memory, point2(0, 0)).map(|path| path.len() - 1)
    }

    fn find_first_blocking_memory(&self, range: Range<usize>) -> Location {
        let test_ix = range.start + (range.len() / 2);
        if test_ix == range.start {
            self.locations[test_ix]
        } else if self.find_path(test_ix).is_some() {
            self.find_first_blocking_memory(test_ix..range.end)
        } else {
            self.find_first_blocking_memory(range.start..test_ix)
        }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let locations: Vec<_> = lines.filter_map(|line| line.parse().ok()).collect();
        let size = if locations.len() >= 1024 { 71 } else { 7 };
        let start_take = if locations.len() >= 1024 { 1024 } else { 12 };
        Day { locations, size, start_take }
    }

    fn calculate_part1(&self) -> Self::Output { self.find_path(self.start_take).unwrap() }

    fn calculate_part2(&self) -> Self::Output {
        let found = self.find_first_blocking_memory(self.start_take..self.locations.len());
        println!("Found: {:?}", found);
        (found.x() * 100 + found.y()) as usize
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 18, example1 => 22, 601 );
    day_test!( 18 => 348, 5444 );
}
