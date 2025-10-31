#![feature(test)]

use advent_lib::{direction::CardinalDirection, grid::Grid, parsing::single_digit, *};
use nom_parse_macros::parse_from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[parse_from(map(single_digit(), |b| (b - b'0') as i32))]
struct Digit(i32);

fn step(grid: &mut Grid<Digit>) {
    grid.entries_mut().for_each(|(_, val)| val.0 += 1);
    loop {
        let locations = Vec::from_iter(
            grid.entries_mut()
                .filter(|(_, value)| value.0 > 9)
                .map(|(loc, _)| loc),
        );
        if locations.is_empty() {
            break;
        }
        for loc in locations {
            grid[loc].0 -= 1000;
            for dir in CardinalDirection::ALL {
                if let Some(val) = grid.get_mut(loc + dir) {
                    val.0 += 1;
                }
            }
        }
    }
    grid.entries_mut()
        .filter(|(_, val)| val.0 < 0)
        .for_each(|(_, val)| val.0 = 0);
}

fn calculate_part1(input: &ParsedInput) -> usize {
    let mut grid = input.clone();
    let mut count = 0;
    for _ in 0..100 {
        step(&mut grid);
        count += grid.entries().filter(|(_, val)| val.0 == 0).count()
    }
    count
}

fn calculate_part2(input: &ParsedInput) -> u64 {
    let mut grid = input.clone();
    for round in 1.. {
        step(&mut grid);
        if grid.entries().all(|(_, val)| val.0 == 0) {
            return round;
        }
    }
    unreachable!()
}

day_main!(Grid<Digit>);

day_test!( 11, example => 1656, 195 );
day_test!( 11 => 1599, 418 );
