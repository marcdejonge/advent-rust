#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::parse_u64;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use rayon::prelude::*;
use std::str::FromStr;

struct Day {
    puzzles: Vec<Puzzle>,
}

impl Day {
    fn sum_of_targets(&self, allow_concat: bool) -> u64 {
        self.puzzles
            .par_iter()
            .filter(|p| can_make_target(p.target, &p.input, allow_concat))
            .map(|p| p.target)
            .sum()
    }
}

struct Puzzle {
    target: u64,
    input: Vec<u64>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        separated_pair(parse_u64, tag(": "), separated_list1(tag(" "), parse_u64))(line)
            .map(|(_, (target, input))| Puzzle { target, input })
            .map_err(|_| ())
    }
}

fn can_make_target(target: u64, input: &[u64], allow_concat: bool) -> bool {
    let (&nr, input) = input.split_last().unwrap();
    if nr > target {
        return false;
    } else if input.is_empty() {
        return nr == target;
    } else if nr == 0 {
        return can_make_target(target, input, allow_concat);
    }

    (allow_concat && can_make_target_concat(target, input, nr, allow_concat))
        || can_make_target_multiply(target, nr, input, allow_concat)
        || can_make_target(target - nr, input, allow_concat)
}

fn can_make_target_concat(target: u64, input: &[u64], nr: u64, allow_concat: bool) -> bool {
    let decimal_size = 10u64.pow(nr.ilog10() + 1);
    let rest = target - nr;
    let (front_part, rem) = (rest / decimal_size, rest % decimal_size);
    rem == 0 && can_make_target(front_part, input, allow_concat)
}

fn can_make_target_multiply(target: u64, nr: u64, input: &[u64], allow_concat: bool) -> bool {
    let (div, rem) = (target / nr, target % nr);
    rem == 0 && can_make_target(div, input, allow_concat)
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { puzzles: lines.map(|s| s.parse()).filter_map(Result::ok).collect() }
    }
    fn calculate_part1(&self) -> Self::Output { self.sum_of_targets(false) }
    fn calculate_part2(&self) -> Self::Output { self.sum_of_targets(true) }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 7, example1 => 3749, 11387 );
    day_test!( 7 => 945512582195, 271691107779347);
}