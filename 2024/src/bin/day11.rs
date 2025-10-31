#![feature(test)]

use advent_lib::*;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;

type Count = u64;

#[parse_from(separated_list1(space1, u64))]
struct Input {
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
            let split = 10u64.pow(start.ilog10().div_ceil(2));
            self.how_many_splits(start / split, times - 1)
                + self.how_many_splits(start % split, times - 1)
        } else {
            self.how_many_splits(start * 2024, times - 1)
        };
        self.memory.insert((start, times), answer);
        answer
    }
}

fn calculate_part1(input: &Input) -> u64 {
    let mut memoized = Memoized::new();
    input.starting_numbers.iter().map(|&n| memoized.how_many_splits(n, 25)).sum()
}
fn calculate_part2(input: &Input) -> u64 {
    let mut memoized = Memoized::new();
    input.starting_numbers.iter().map(|&n| memoized.how_many_splits(n, 75)).sum()
}

day_main!(Input);
day_test!( 11, example1 => 55312, 65601038650482 );
day_test!( 11 => 217443, 257246536026785);
