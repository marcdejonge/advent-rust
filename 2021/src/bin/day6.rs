#![feature(test)]

use advent_lib::{iter_utils::SumWith, *};
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;

#[parse_from(separated_list1(",", i32))]
struct Input(Vec<i32>);

impl Input {
    fn count_all_fishes(&self, generation: i32) -> u64 {
        let mut memoize = Memoize::default();
        self.0
            .iter()
            .sum_with(|s| memoize.count_fishes(generation + 8 - s))
    }
}

#[derive(Default)]
struct Memoize {
    count_fishes: FxHashMap<i32, u64>,
}

impl Memoize {
    fn count_fishes(&mut self, generation: i32) -> u64 {
        if let Some(result) = self.count_fishes.get(&generation) {
            *result
        } else {
            let result = 1
                + (0..=generation - 9)
                    .rev()
                    .step_by(7)
                    .sum_with(|next_gen| self.count_fishes(next_gen));
            self.count_fishes.insert(generation, result);
            result
        }
    }
}

fn calculate_part1(input: &Input) -> u64 {
    input.count_all_fishes(80)
}

fn calculate_part2(input: &Input) -> u64 {
    input.count_all_fishes(256)
}

day_main!(Input);

day_test!( 6, example => 5934, 26984457539 );
day_test!( 6 => 379414, 1705008653296 );
