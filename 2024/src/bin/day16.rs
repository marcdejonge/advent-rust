#![feature(test)]

use crate::Block::*;
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::grid::{Grid, Location};
use advent_lib::*;
use advent_macros::FromRepr;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use priority_queue::PriorityQueue;
use std::cmp::{Reverse, min};
use std::collections::hash_map::Entry::Vacant;
use std::ops::Neg;

#[parse_from(Grid::parse)]
struct Input {
    grid: Grid<Block>,
    #[derived(grid.find(|&b| b == Start).expect("Start not found"))]
    start: Location,
    #[derived(grid.find(|&b| b == End).expect("End not found"))]
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

impl Input {
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
        for dir in Direction::ALL {
            let next_pos = Step::new(step.loc + dir, dir.neg());
            if let Some(next_score) = visited.get(&next_pos)
                && *next_score < current_score
            {
                Self::fill_seat_grid(next_pos, seat_grid, visited);
            }
        }
    }
}

fn calculate_part1(input: &Input) -> u32 { input.find_all_paths().0 }

fn calculate_part2(input: &Input) -> usize {
    let visited = input.find_all_paths().1;

    let mut seat_grid = input.grid.clone();
    Direction::ALL
        .into_iter()
        .map(|dir| Step::new(input.end, dir))
        .for_each(|step| {
            if visited.contains_key(&step) {
                Input::fill_seat_grid(step, &mut seat_grid, &visited);
            }
        });
    seat_grid.values().filter(|&&b| b == Seat).count()
}

day_main!(Input);
day_test!( 16, example1 => 7036, 45 );
day_test!( 16, example2 => 11048, 64 );
day_test!( 16 => 88416, 442);
