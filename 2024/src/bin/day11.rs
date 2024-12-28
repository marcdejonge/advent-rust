#![feature(test)]

use advent_lib::day::*;
use fxhash::FxHashMap;
use nom::character::complete;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Parser;

type Count = u64;

struct Day {
    starting_numbers: Vec<u64>,
}

struct Memoized {
    memory: FxHashMap<(u64, u64), Count>,
}

impl Memoized {
    fn new() -> Memoized { Memoized { memory: FxHashMap::default() } }

    fn how_many_splits(&mut self, start: u64, times: u64) -> Count {
        if let Some(answer) = self.memory.get(&(start, times)) {
            return *answer;
        }

        let answer = if times == 0 {
            1
        } else if start == 0 {
            self.how_many_splits(1, times - 1)
        } else if start.ilog10() % 2 == 1 {
            let split = 10u64.pow((start.ilog10() + 1) / 2);
            self.how_many_splits(start / split, times - 1)
                + self.how_many_splits(start % split, times - 1)
        } else {
            self.how_many_splits(start * 2024, times - 1)
        };
        self.memory.insert((start, times), answer);
        answer
    }
}

impl ExecutableDay for Day {
    type Output = Count;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(complete::space1, complete::u64),
            |starting_numbers| Day { starting_numbers },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut memoized = Memoized::new();
        self.starting_numbers.iter().map(|&n| memoized.how_many_splits(n, 25)).sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut memoized = Memoized::new();
        self.starting_numbers.iter().map(|&n| memoized.how_many_splits(n, 75)).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 11, example1 => 55312, 65601038650482 );
    day_test!( 11 => 217443, 257246536026785);
}
