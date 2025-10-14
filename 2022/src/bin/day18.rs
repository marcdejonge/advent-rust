#![feature(test)]

use advent_lib::geometry::{unit_vector, vector3, FindBoundingBox};
use advent_lib::search::depth_first_search;
use advent_lib::*;
use fxhash::{FxBuildHasher, FxHashSet};
use std::collections::HashSet;

type Point = advent_lib::geometry::Point<3, i8>;
type Vector = advent_lib::geometry::Vector<3, i8>;

const DIRECTIONS: [Vector; 6] = [
    vector3(1, 0, 0),
    vector3(-1, 0, 0),
    vector3(0, 1, 0),
    vector3(0, -1, 0),
    vector3(0, 0, 1),
    vector3(0, 0, -1),
];

fn calculate_part1(points: &FxHashSet<Point>) -> usize {
    points
        .iter()
        .map(|p| DIRECTIONS.iter().filter(|&dir| !points.contains(&(*p - *dir))).count())
        .sum()
}

fn calculate_part2(points: &FxHashSet<Point>) -> usize {
    let mut rect = points.iter().cloned().enclosing_rect().unwrap();
    // Make the rectangle 1 step bigger to make sure we can wrap all the sides
    rect.expand(unit_vector());

    let mut outside_blocks = HashSet::with_capacity_and_hasher(4096, FxBuildHasher::default());
    depth_first_search(
        rect.min_point(),
        |p| DIRECTIONS.iter().map(move |dir| p + *dir),
        |p| {
            if rect.contains_inclusive(&p) && !outside_blocks.contains(&p) && !points.contains(&p) {
                outside_blocks.insert(p);
                true
            } else {
                false
            }
        },
    );

    points
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

day_main!();
day_test!( 18, example => 64, 58 );
day_test!( 18 => 4320, 2456 );
