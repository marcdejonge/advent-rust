#![feature(test)]

use crate::Block::*;
use advent_lib::day::*;
use advent_lib::direction::Direction::*;
use advent_lib::direction::{Direction, ALL_DIRECTIONS};
use advent_lib::grid::{Grid, Location};
use advent_macros::FromRepr;
use fxhash::FxHashMap;
use priority_queue::PriorityQueue;
use std::cmp::{min, Reverse};
use std::collections::hash_map::Entry::Vacant;
use std::ops::Neg;

struct Day {
    grid: Grid<Block>,
    start: Location,
    end: Location,
}

#[derive(FromRepr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Block {
    Empty = b'.',
    Wall = b'#',
    Start = b'S',
    End = b'E',
    Seat = b'O',
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Step {
    loc: Location,
    dir: Direction,
}

impl Step {
    fn new(loc: Location, dir: Direction) -> Self { Step { loc, dir } }
}

impl Day {
    fn find_all_paths(&self) -> (u32, FxHashMap<Step, u32>) {
        let mut visited = FxHashMap::default();
        let mut queue = PriorityQueue::new();
        let start = Step::new(self.start, East);
        queue.push(start, Reverse(0));

        let mut end_score = u32::MAX;

        while let Some((step, Reverse(score))) = queue.pop() {
            if score > min(*visited.get(&step).unwrap_or(&u32::MAX), end_score) {
                continue; // Worse route, skip it
            }
            visited.insert(step, score);
            if step.loc == self.end {
                end_score = min(score, end_score);
                continue;
            }

            let next_loc = step.loc + step.dir;
            if self.grid.get(next_loc) != Some(&Wall) {
                queue.push(Step::new(next_loc, step.dir), Reverse(score + 1));
            }
            for next_dir in [step.dir.turn_left(), step.dir.turn_right()] {
                let next_score = score + 1000;
                let next_step = Step::new(step.loc, next_dir);
                if let Vacant(entry) = visited.entry(next_step) {
                    entry.insert(next_score);
                    if self.grid.get(step.loc + next_dir) != Some(&Wall) {
                        queue.push(
                            Step::new(step.loc + next_dir, next_dir),
                            Reverse(next_score + 1),
                        );
                    }
                }
            }
        }

        (end_score, visited)
    }

    fn fill_seat_grid(step: Step, seat_grid: &mut Grid<Block>, visited: &FxHashMap<Step, u32>) {
        seat_grid[step.loc] = Seat;
        let current_score = visited[&step];
        for dir in ALL_DIRECTIONS {
            let next_pos = Step::new(step.loc + dir, dir.neg());
            if let Some(next_score) = visited.get(&next_pos) {
                if *next_score < current_score {
                    Self::fill_seat_grid(next_pos, seat_grid, visited);
                }
            }
        }
    }
}

impl ExecutableDay for Day {
    type Output = u32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let grid = Grid::from(lines);
        let start = grid.find(|&b| b == Start).expect("Start not found");
        let end = grid.find(|&b| b == End).expect("End not found");
        Day { grid, start, end }
    }

    fn calculate_part1(&self) -> Self::Output { self.find_all_paths().0 }

    fn calculate_part2(&self) -> Self::Output {
        let visited = self.find_all_paths().1;

        let mut seat_grid = self.grid.clone();
        ALL_DIRECTIONS.iter().map(|&dir| Step::new(self.end, dir)).for_each(|step| {
            if visited.contains_key(&step) {
                Self::fill_seat_grid(step, &mut seat_grid, &visited);
            }
        });
        seat_grid.values().filter(|&&b| b == Seat).count() as u32
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 16, example1 => 7036, 45 );
    day_test!( 16, example2 => 11048, 64 );
    day_test!( 16 => 88416, 442);
}