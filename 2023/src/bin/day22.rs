#![feature(test)]

use std::collections::VecDeque;

use advent_lib::day::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::key::Key;
use advent_lib::lines::LineSegment;
use advent_macros::parsable;
use fxhash::{FxHashMap, FxHashSet};

#[parsable(map(parsable_pair(tag(b"~")), |(start, end)| LineSegment{ start, end }))]
struct Brick(LineSegment<3, i64>);
type BrickSet = bit_set::BitSet<usize>;

impl Brick {
    fn all_xy(&self) -> impl Iterator<Item = Point<2, i64>> {
        let x_range = self.0.start.x()..=self.0.end.x();
        let y_range = self.0.start.y()..=self.0.end.y();
        x_range.flat_map(move |x| y_range.clone().map(move |y| point2(x, y)))
    }

    fn height(&self) -> i64 { self.0.end.z() - self.0.start.z() + 1 }
}

#[parsable(map(separated_lines1(), |mut bricks: Vec<Brick>| {
    bricks.sort_by_key(|brick| brick.0.start.z());
    bricks
}))]
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
            let brick_height = brick.height();
            let high_points: Vec<_> =
                brick.all_xy().filter_map(|p| height_grid.get(&p).cloned()).collect();

            if high_points.is_empty() {
                // No current points found, so put it on the ground
                brick.all_xy().for_each(|loc| {
                    height_grid.insert(loc, (brick_height, index.into()));
                });
            } else {
                let (max_height, _) = *high_points.iter().max_by_key(|(h, _)| *h).unwrap();
                brick.all_xy().for_each(|loc| {
                    height_grid.insert(loc, (max_height + brick_height, index.into()));
                });

                let supported_by: FxHashSet<Key> = high_points
                    .iter()
                    .filter(|(h, _)| *h == max_height)
                    .map(|(_, key)| *key)
                    .collect();
                handle_supported_by(index.into(), supported_by);
            }
        }
    }
}

struct BrickSupport {
    supports: FxHashMap<Key, FxHashSet<Key>>,
    supported_by: FxHashMap<Key, FxHashSet<Key>>,
}

impl BrickSupport {
    fn new(size: usize) -> BrickSupport {
        let mut supports = FxHashMap::default();
        let mut supported_by = FxHashMap::default();

        for ix in 0..size {
            let key = Key::from(ix);
            supports.insert(key, Default::default());
            supported_by.insert(key, Default::default());
        }

        BrickSupport { supports, supported_by }
    }

    fn register_support(&mut self, key: Key, supported_by: FxHashSet<Key>) {
        for supported_key in &supported_by {
            self.supports.get_mut(supported_key).unwrap().insert(key);
        }
        self.supported_by.insert(key, supported_by);
    }

    fn is_supported(&self, key: &Key, dropped_keys: &BrickSet) -> bool {
        if let Some(supported_by) = self.supported_by.get(key) {
            supported_by.iter().any(|key| !dropped_keys.contains((*key).into()))
        } else {
            false
        }
    }
}

#[derive(Default)]
struct BrickDropped {
    // Key -> (dropped bricks, supporting bricks not dropped)
    bricks: FxHashMap<Key, (BrickSet, BrickSet)>,
}

impl BrickDropped {
    fn calc_dropped_bricks(&mut self, support: &BrickSupport, key: &Key) -> usize {
        if let Some((dropped, _)) = self.bricks.get(key) {
            return dropped.len() - 1;
        }
        // println!("Calculating for {key}");

        let mut dropped_bricks = BrickSet::default();
        dropped_bricks.reserve_len_exact(support.supports.len());
        let mut supporting_bricks = BrickSet::default();
        supporting_bricks.reserve_len_exact(support.supports.len());
        dropped_bricks.insert((*key).into());

        let mut stack = VecDeque::<Key>::with_capacity(256);
        if let Some(supports) = support.supports.get(key) {
            supports.iter().for_each(|support| {
                stack.push_back(*support);
                supporting_bricks.insert((*support).into());
            });
        }

        while let Some(to_drop) = stack.pop_front() {
            if !support.is_supported(&to_drop, &dropped_bricks) {
                self.calc_dropped_bricks(support, &to_drop);

                let (next_dropped_bricks, next_supported_bricks) =
                    self.bricks.get(&to_drop).unwrap();
                dropped_bricks.union_with(next_dropped_bricks);
                supporting_bricks.difference_with(next_dropped_bricks);

                for next_supported_brick in next_supported_bricks {
                    if supporting_bricks.contains(next_supported_brick) {
                        stack.push_back(next_supported_brick.into())
                    } else {
                        supporting_bricks.insert(next_supported_brick);
                    }
                }
            }
        }

        let count = dropped_bricks.len() - 1;
        // println!("Saved {count} for {key}:\n{dropped_keys:?}");
        self.bricks.insert(*key, (dropped_bricks, supporting_bricks));
        count
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn calculate_part1(&self) -> Self::Output {
        let mut single_support = BrickSet::default();
        self.drop_bricks(|_, supported_by| {
            if supported_by.len() == 1 {
                single_support.insert((*supported_by.iter().next().unwrap()).into());
            }
        });
        self.starting_bricks.len() - single_support.len()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut brick_support = BrickSupport::new(self.starting_bricks.len());
        let mut bricks_dropped = BrickDropped::default();
        self.drop_bricks(|key, supported_by| brick_support.register_support(key, supported_by));

        (0..self.starting_bricks.len())
            .rev()
            .map(|index| {
                let index_key = Key::from(index);
                bricks_dropped.calc_dropped_bricks(&brick_support, &index_key)
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
