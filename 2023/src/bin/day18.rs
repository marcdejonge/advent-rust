/*

*/
#![feature(test)]

use prse_derive::parse;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use Direction::*;

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

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let (dig_plan1, dig_plan2): (Vec<_>, Vec<_>) = lines
            .map(|line| {
                let (direction, steps, color): (_, _, String) = parse!(line, "{} {} (#{})");
                let big_steps = i64::from_str_radix(&color[0..5], 16)
                    .expect("Expect a valid hexadecimal value {color}");
                let big_direction = Direction::from(color.chars().last().unwrap() as u8);
                (
                    DigCommand { direction, steps },
                    DigCommand { direction: big_direction, steps: big_steps },
                )
            })
            .unzip();
        Day { dig_plan1, dig_plan2 }
    }

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
