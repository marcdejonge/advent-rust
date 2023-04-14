#![feature(test)]
use advent_lib::day::*;
use advent_lib::iter_utils::*;
use std::collections::BinaryHeap;

struct Day {
    sorted_sums: Vec<i32>,
}

impl ExecutableDay for Day {
    type Output = i32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            sorted_sums: lines
                .chunk_by("".to_owned())
                .map(|v| v.iter().map(|line| line.parse::<i32>().unwrap()).sum())
                .collect::<BinaryHeap<_>>()
                .into_sorted_vec(),
        }
    }

    fn calculate_part1(&self) -> Self::Output { self.sorted_sums.iter().rev().take(1).sum() }

    fn calculate_part2(&self) -> Self::Output { self.sorted_sums.iter().rev().take(3).sum() }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example => 24000, 45000 );
    day_test!( 1 => 68292, 203203);
}
