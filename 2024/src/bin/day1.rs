#![feature(test)]

use advent_lib::day::*;

struct Day;

impl ExecutableDay for Day {
    type Output = u32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self { Day }
    fn calculate_part1(&self) -> Self::Output { 0 }
    fn calculate_part2(&self) -> Self::Output { 0 }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example1 => 1, 1 );
    day_test!( 1, example2 => 1, 1 );
    day_test!( 1 => 1, 1);
}
