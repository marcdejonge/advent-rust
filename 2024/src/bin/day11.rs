#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use fxhash::FxHashMap;

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

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            starting_numbers: lines
                .single()
                .unwrap()
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect(),
        }
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
