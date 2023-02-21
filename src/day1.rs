use std::collections::BinaryHeap;

use crate::{ExecutableDay, execute_day};
use crate::iter_utils::Chunkable;

pub(crate) fn execute() { execute_day::<Day1>() }

struct Day1 {}

impl ExecutableDay for Day1 {
    type Input = Vec<i32>;
    type Output = i32;

    fn get_code() -> i32 { 1 }

    fn parse_input(file_input: &str) -> Vec<i32> {
        file_input.lines()
            .chunk_by("")
            .map(|v| v.iter().map(|&line| line.parse::<i32>().unwrap()).sum())
            .collect::<BinaryHeap<_>>()
            .into_sorted_vec()
    }

    fn calculate_part1(input: &Vec<i32>) -> i32 {
        input.iter().rev().take(1).sum()
    }

    fn calculate_part2(input: &Vec<i32>) -> i32 {
        input.iter().rev().take(3).sum()
    }
}
