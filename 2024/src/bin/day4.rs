#![feature(test)]

use advent_lib::day_main;
use advent_lib::geometry::{vector2, Vector};
use advent_lib::grid::{Grid, Location};

type Step = Vector<2, i32>;

fn check_ms_around_a(grid: &Grid<u8>, location: Location, first: Step, second: Step) -> bool {
    match grid.get(location + first) {
        Some(&b'M') => grid.get(location + second) == Some(&b'S'),
        Some(&b'S') => grid.get(location + second) == Some(&b'M'),
        _ => false,
    }
}

fn calculate_part1(grid: &Grid<u8>) -> usize {
    const DIRECTIONS: [Step; 8] = [
        vector2(1, -1),
        vector2(1, 0),
        vector2(1, 1),
        vector2(0, -1),
        vector2(0, 1),
        vector2(-1, -1),
        vector2(-1, 0),
        vector2(-1, 1),
    ];

    grid.entries()
        .filter(|(_, &char)| char == b'X')
        .map(|(location, _)| {
            DIRECTIONS
                .iter()
                .filter(|&&dir| {
                    grid.get(location + dir) == Some(&b'M')
                        && grid.get(location + dir * 2) == Some(&b'A')
                        && grid.get(location + dir * 3) == Some(&b'S')
                })
                .count()
        })
        .sum()
}

fn calculate_part2(grid: &Grid<u8>) -> usize {
    grid.entries()
        .filter(|(location, &char)| {
            char == b'A'
                && check_ms_around_a(grid, *location, vector2(1, -1), vector2(-1, 1))
                && check_ms_around_a(grid, *location, vector2(1, 1), vector2(-1, -1))
        })
        .count()
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example1 => 18, 9 );
    day_test!( 4 => 2530, 1921);
}
