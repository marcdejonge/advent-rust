#![feature(test)]

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::geometry::{unit_vector, vector3, FindBoundingBox};
use advent_lib::search::depth_first_search;
use fxhash::FxBuildHasher;
use std::collections::HashSet;

type Point = advent_lib::geometry::Point<3, i8>;
type Vector = advent_lib::geometry::Vector<3, i8>;

struct Day {
    points: HashSet<Point, FxBuildHasher>,
}

const DIRECTIONS: [Vector; 6] = [
    vector3(1, 0, 0),
    vector3(-1, 0, 0),
    vector3(0, 1, 0),
    vector3(0, -1, 0),
    vector3(0, 0, 1),
    vector3(0, 0, -1),
];

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut points = HashSet::with_capacity_and_hasher(4096, FxBuildHasher::default());
        for line in lines {
            points.insert(line.parse().expect("Could not parse line"));
        }
        Day { points }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.points
            .iter()
            .map(|p| DIRECTIONS.iter().filter(|&dir| !self.points.contains(&(*p - *dir))).count())
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut rect = self.points.iter().cloned().enclosing_rect().unwrap();
        // Make the rectangle 1 step bigger to make sure we can wrap all the sides
        rect.expand(unit_vector());

        let mut outside_blocks = HashSet::with_capacity_and_hasher(4096, FxBuildHasher::default());
        depth_first_search(
            rect.min_point(),
            |p| DIRECTIONS.iter().map(move |dir| p + *dir),
            |p| {
                if rect.contains_inclusive(&p)
                    && !outside_blocks.contains(&p)
                    && !self.points.contains(&p)
                {
                    outside_blocks.insert(p);
                    true
                } else {
                    false
                }
            },
        );

        self.points
            .iter()
            .map(|p| {
                DIRECTIONS
                    .iter()
                    .filter(|&dir| {
                        let neighbour = *p + *dir;
                        outside_blocks.contains(&neighbour)
                    })
                    .count()
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 18, example => 64, 58 );
    day_test!( 18 => 4320, 2456 );
}
