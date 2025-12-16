#![feature(test)]
#![feature(iter_collect_into)]

use advent_lib::{disjoint_set::DisjointSet, *};
use itertools::Itertools;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

type Point = advent_lib::geometry::Point<3, i64>;
type DistancedPair = (i64, usize, usize);

#[parse_from]
struct Input {
    points: Vec<Point>,
    #[derived(sorted_distances(&points))]
    sorted_distances: Vec<DistancedPair>,
    #[derived(points.len())]
    len: usize,
}

fn sorted_distances(points: &[Point]) -> Vec<DistancedPair> {
    let mut result = Vec::with_capacity(points.len() * points.len() / 2);

    points
        .iter()
        .enumerate()
        .flat_map(|(left_ix, left)| {
            points.iter().enumerate().skip(left_ix + 1).map(move |(right_ix, right)| {
                let dist_vec = *right - *left;
                let distance = dist_vec.x() * dist_vec.x()
                    + dist_vec.y() * dist_vec.y()
                    + dist_vec.z() * dist_vec.z();
                (distance, left_ix, right_ix)
            })
        })
        .collect_into(&mut result);

    result.par_sort_unstable();

    result
}

fn calculate_part1(input: &Input) -> usize {
    let mut links = DisjointSet::with_len(input.len);
    for &(_, left_ix, right_ix) in
        input.sorted_distances.iter().take(if input.len < 100 { 10 } else { 1000 })
    {
        links.join(left_ix, right_ix);
    }

    links.sets().map(|set| set.len()).sorted().rev().take(3).product()
}

fn calculate_part2(input: &Input) -> i64 {
    let mut links = DisjointSet::with_len(input.len);
    let &(_, left_ix, right_ix) = input
        .sorted_distances
        .iter()
        .filter(|&&(_, left_ix, right_ix)| links.join(left_ix, right_ix))
        .nth(input.len - 2) // The 0th item already joined the first 2, so subtract 2
        .unwrap();

    input.points[left_ix].x() * input.points[right_ix].x()
}

day_main!(Input);

day_test!( 8, example => 40, 25272 );
day_test!( 8 => 117000, 8368033065 );
