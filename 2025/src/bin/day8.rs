#![feature(test)]

use advent_lib::{disjoint_set::DisjointSet, *};
use itertools::Itertools;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

type Point = advent_lib::geometry::Point<3, i64>;

#[parse_from]
struct Input {
    points: Vec<Point>,
    #[derived(sorted_distances(&points))]
    sorted_distances: Vec<(usize, usize, i64)>,
    #[derived(points.len())]
    len: usize,
}

fn sorted_distances(points: &[Point]) -> Vec<(usize, usize, i64)> {
    let mut result: Vec<(usize, usize, i64)> = (0..points.len())
        .par_bridge()
        .flat_map_iter(|left_ix| {
            ((left_ix + 1)..points.len()).map(move |right_ix| {
                let dist_vec = points[right_ix] - points[left_ix];
                let dist = (dist_vec.x() * dist_vec.x()
                    + dist_vec.y() * dist_vec.y()
                    + dist_vec.z() * dist_vec.z())
                .isqrt();
                (left_ix, right_ix, dist)
            })
        })
        .collect();

    result.par_sort_unstable_by_key(|&(_, _, weight)| weight);

    result
}

fn calculate_part1(input: &Input) -> usize {
    let mut links = DisjointSet::with_len(input.len);
    for &(left_ix, right_ix, _) in
        input.sorted_distances.iter().take(if input.len < 100 { 10 } else { 1000 })
    {
        links.join(left_ix, right_ix);
    }

    links.sets().map(|set| set.len()).sorted().rev().take(3).product()
}

fn calculate_part2(input: &Input) -> i64 {
    let mut links = DisjointSet::with_len(input.len);
    let &(left_ix, right_ix, _) = input
        .sorted_distances
        .iter()
        .filter(|&&(left_ix, right_ix, _)| links.join(left_ix, right_ix))
        .nth(input.len - 2) // The 0th item already joined the first 2, so subtract 2
        .unwrap();

    input.points[left_ix].x() * input.points[right_ix].x()
}

day_main!(Input);

day_test!( 8, example => 40, 25272 );
day_test!( 8 => 117000, 8368033065 );
