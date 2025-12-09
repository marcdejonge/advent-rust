#![feature(test)]
#![feature(iter_map_windows)]

use advent_lib::{geometry::BoundingBox, lines::LineSegment, *};
use rayon::prelude::*;

type Point = advent_lib::geometry::Point<2, i32>;

fn find_area<F>(points: &[Point], predicate: F) -> i64
where
    F: Fn(&BoundingBox<2, i32>) -> bool + Send + Sync,
{
    points
        .iter()
        .enumerate()
        .par_bridge()
        .flat_map_iter(|(ix, left)| {
            points.iter().skip(ix).map(move |right| BoundingBox::from(*left, *right))
        })
        .filter(predicate)
        .map(|bb| {
            let v = bb.max_point() - bb.min_point();
            (v.x() + 1) as i64 * (v.y() + 1) as i64
        })
        .max()
        .unwrap()
}

fn calculate_part1(points: &[Point]) -> i64 { find_area(points, |_| true) }

fn calculate_part2(points: &[Point]) -> i64 {
    let lines: Vec<_> = points
        .iter()
        .chain(points.iter().take(1))
        .map_windows(|[p1, p2]| LineSegment::from((**p1, **p2)))
        .collect();

    find_area(points, |bb| {
        !lines.iter().any(|line| bb.line_crosses(&line))
    })
}

day_main!(Vec<Point>);

day_test!( 9, example => 50, 24 );
day_test!( 9 => 4774877510, 1560475800 );
