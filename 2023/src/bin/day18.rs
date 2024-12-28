#![feature(test)]

extern crate core;

use advent_lib::day::*;
use advent_lib::direction::*;
use advent_lib::rgb::*;
use advent_macros::parsable;
use Direction::*;

#[parsable(
    map(
        separated_list1(
            line_ending,
            map(
                tuple((
                    direction_parser,
                    space1,
                    i64,
                    space1,
                    delimited(tag(b"("), RGB::parser(),tag(b")")),                    
                )),
                |(direction, _, steps, _, color)| (
                    DigCommand { direction, steps },
                    DigCommand { direction: Direction::from(color.blue & 3), steps: (u32::from(color) >> 4) as i64 },
                )
            )
        ),
        |list| list.into_iter().unzip()
    )
)]
struct Day {
    dig_plan1: Vec<DigCommand>,
    dig_plan2: Vec<DigCommand>,
}

#[derive(Debug)]
struct DigCommand {
    direction: Direction,
    steps: i64,
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

impl ExecutableDay for Day {
    type Output = i64;

    fn calculate_part1(&self) -> Self::Output { calculate_area(&self.dig_plan1) }

    fn calculate_part2(&self) -> Self::Output { calculate_area(&self.dig_plan2) }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 18, example => 62, 952408144115 );
    day_test!( 18 => 53300, 64294334780659 );
}
