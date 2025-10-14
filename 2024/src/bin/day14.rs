#![feature(test)]

use advent_lib::direction::CardinalDirections;
use advent_lib::geometry::{vector2, vector4, Point, Vector};
use advent_lib::grid::Grid;
use advent_lib::*;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[parse_from(separated_list1(line_ending, ()))]
struct Input {
    robots: Vec<Robot>,
}

#[derive(Clone)]
#[parse_from(match "p={} v={}")]
struct Robot {
    p: Point<2, i32>,
    v: Vector<2, i32>,
}

fn calculate_part1(input: &Input) -> u32 {
    let size = if input.robots.len() < 20 { vector2(11, 7) } else { vector2(101, 103) };

    let counts: Vector<4, u32> = input
        .robots
        .iter()
        .map(|robot| {
            let p = (robot.p + robot.v * 100) % size;
            let in_left = p.x() < (size.x() / 2);
            let in_right = p.x() > (size.x() / 2);
            let in_top = p.y() < (size.y() / 2);
            let in_bottom = p.y() > (size.y() / 2);
            vector4(
                (in_left && in_top) as u32,
                (in_left && in_bottom) as u32,
                (in_right && in_top) as u32,
                (in_right && in_bottom) as u32,
            )
        })
        .sum();

    counts.x() * counts.y() * counts.z() * counts.w()
}

fn calculate_part2(input: &Input) -> i32 {
    if input.robots.len() != 500 {
        return 0;
    }
    let size = vector2(101, 103);

    let (_, time, grid) = (0..(size.x() * size.y()))
        .par_bridge()
        .map(|t| {
            let mut grid = Grid::new_default(b' ', size.x(), size.y());
            input
                .robots
                .iter()
                .map(|robot| (robot.p + robot.v * t) % size)
                .for_each(|p| grid[p] = b'#');
            let neighbours: usize = grid
                .entries()
                .filter(|&(_, &c)| c == b'#')
                .map(|(p, _)| {
                    CardinalDirections::ALL
                        .map(|d| p + d)
                        .iter()
                        .filter(|&&p| grid.get(p) == Some(&b'#'))
                        .count()
                })
                .sum();
            (neighbours, t, grid)
        })
        .max_by(|(n1, _, _), (n2, _, _)| n1.cmp(n2))
        .unwrap();

    println!("{:?}", grid);
    time
}

day_main!();
day_test!( 14, example1 => 12, 0 );
day_test!( 14 => 218619120, 7055);
