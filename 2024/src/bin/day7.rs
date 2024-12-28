#![feature(test)]

use advent_lib::day::*;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use rayon::prelude::*;

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

fn can_make_target(target: u64, input: &[u64], allow_concat: bool) -> bool {
    let (&nr, input) = input.split_last().unwrap();
    if nr > target {
        // If the current number is larger than the target, then this can't be a working solution
        return false;
    } else if input.is_empty() {
        // If there are no numbers left after this, we should have hit the target now
        return nr == target;
    } else if nr == target {
        // If we've hit the target, but there are more numbers; then there should be a zero we can
        // multiply all with (or add if it's all zeroes), or it should be all ones to multiply with
        return input.contains(&0) || input.iter().all(|&x| x == 1);
    } else if nr == 0 {
        // If the current number is 0, and we've not hit the target, we have to add it
        return can_make_target(target, input, allow_concat);
    }

    (allow_concat && can_make_target_concat(target, input, nr, allow_concat))
        || can_make_target_multiply(target, nr, input, allow_concat)
        || can_make_target(target - nr, input, allow_concat)
}

fn can_make_target_concat(target: u64, input: &[u64], nr: u64, allow_concat: bool) -> bool {
    let decimal_size = if nr == 0 { 10 } else { 10u64.pow(nr.ilog10() + 1) };
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

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(
                line_ending,
                map(
                    separated_pair(
                        complete::u64,
                        tag(b": "),
                        separated_list1(tag(b" "), complete::u64),
                    ),
                    |(target, input)| Puzzle { target, input },
                ),
            ),
            |puzzles| Day { puzzles },
        )
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
