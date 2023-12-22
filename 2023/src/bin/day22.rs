#![feature(test)]

use bit_set::BitSet;
use fxhash::{FxHashMap, FxHashSet};
use prse_derive::parse;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;

use advent_lib::day::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::key::Key;

use advent_lib::lines::LineSegment;

type Brick = LineSegment<3, i64>;

fn all_xy(brick: &Brick) -> impl Iterator<Item = Point<2, i64>> {
    let x_range = brick.start.x()..=brick.end.x();
    let y_range = brick.start.y()..=brick.end.y();
    x_range.flat_map(move |x| y_range.clone().map(move |y| point2(x, y)))
}

struct Day {
    starting_bricks: Vec<Brick>,
}

impl Day {
    fn drop_bricks<F>(&self, mut handle_supported_by: F)
    where
        F: FnMut(Key, FxHashSet<Key>),
    {
        // Location -> (height, key of brick)
        let mut height_grid: FxHashMap<Point<2, i64>, (i64, Key)> = Default::default();

        for (index, brick) in self.starting_bricks.iter().enumerate() {
            let brick_height = brick.end.z() - brick.start.z() + 1;
            let high_points: Vec<_> =
                all_xy(brick).filter_map(|p| height_grid.get(&p).cloned()).collect();

            if high_points.is_empty() {
                // No current points found, so put it on the ground
                all_xy(brick).for_each(|loc| {
                    height_grid.insert(loc, (brick_height, index.into()));
                });
            } else {
                let (max_height, _) = *high_points.iter().max_by_key(|(h, _)| *h).unwrap();
                all_xy(brick).for_each(|loc| {
                    height_grid.insert(loc, (max_height + brick_height, index.into()));
                });

                let supported_by: FxHashSet<_> = high_points
                    .iter()
                    .filter(|(h, _)| *h == max_height)
                    .map(|(_, key)| *key)
                    .collect();
                handle_supported_by(index.into(), supported_by);
            }
        }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut bricks: Vec<Brick> = lines.map(|line| parse!(line, "{}~{}").into()).collect();
        bricks.sort_by_key(|b| b.start.z());
        Day { starting_bricks: bricks }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut single_support = BitSet::with_capacity(self.starting_bricks.len());
        self.drop_bricks(|_, supported_by| {
            if supported_by.len() == 1 {
                single_support.insert(usize::from(*supported_by.iter().next().unwrap()));
            }
        });
        self.starting_bricks.len() - single_support.len()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut supports: FxHashMap<Key, FxHashSet<Key>> = Default::default();
        let mut supported_by: FxHashMap<Key, FxHashSet<Key>> = Default::default();

        self.drop_bricks(|key, supported_keys| {
            for supported_key in &supported_keys {
                match supports.entry(*supported_key) {
                    Entry::Occupied(mut entry) => entry.get_mut().insert(key),
                    Entry::Vacant(entry) => entry.insert(Default::default()).insert(key),
                };
            }
            supported_by.insert(key, supported_keys);
        });

        (0..self.starting_bricks.len())
            .map(|index| {
                let index_key = Key::from(index);
                let mut dropped_keys = FxHashSet::<Key>::default();
                let mut stack = VecDeque::<Key>::with_capacity(256);
                stack.push_back(index_key);

                while let Some(dropped) = stack.pop_front() {
                    if dropped_keys.insert(dropped) {
                        if let Some(supported_keys) = supports.get(&dropped) {
                            for drop in supported_keys {
                                if supported_by[drop].iter().all(|s| dropped_keys.contains(s)) {
                                    stack.push_back(*drop);
                                }
                            }
                        }
                    }
                }

                dropped_keys.len() - 1 // Don't count the initial dropped one
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 22, example => 5, 7 );
    day_test!( 22 => 457, 79122 );
}
