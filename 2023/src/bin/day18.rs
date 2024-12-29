#![feature(test)]

extern crate core;

use advent_lib::day_main;
use advent_lib::direction::*;
use advent_lib::rgb::*;
use advent_macros::parsable;
use Direction::*;

#[parsable(
    separated_list1(
        line_ending,
        tuple((
            terminated(direction_parser, space1),
            terminated(i64, space1),
            delimited(tag(b"("), RGB::parser(),tag(b")")),
        ))
    )
)]
struct Input {
    lines: Vec<(Direction, i64, RGB)>,
}

#[derive(Debug)]
struct DigCommand {
    direction: Direction,
    steps: i64,
}

fn dig_command_1(input: &Input) -> Vec<DigCommand> {
    input
        .lines
        .iter()
        .map(|&(direction, steps, _)| DigCommand { direction, steps })
        .collect()
}

fn dig_command_2(input: &Input) -> Vec<DigCommand> {
    input
        .lines
        .iter()
        .map(|&(_, _, color)| DigCommand {
            direction: Direction::from(color.blue & 3),
            steps: (u32::from(color) >> 4) as i64,
        })
        .collect()
}

// Simplified from https://www.mathsisfun.com/geometry/area-irregular-polygons.html
fn calculate_area(lines: &[DigCommand]) -> i64 {
    let mut curr_height = 0i64;
    let mut area = 1i64; // With the algorithm below, we always miss the start block
    for DigCommand { direction, steps } in lines {
        match direction {
            North => {
                area += steps; // One of the sides is always outside, so count the steps
                curr_height -= steps;
            }
            East => area -= steps * curr_height,
            South => curr_height += steps,
            West => area += steps * (curr_height + 1),
        }
    }
    area.abs()
}

fn calculate_part1(input: &Input) -> i64 { calculate_area(&dig_command_1(input)) }

fn calculate_part2(input: &Input) -> i64 { calculate_area(&dig_command_2(input)) }

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 18, example => 62, 952408144115 );
    day_test!( 18 => 53300, 64294334780659 );
}
