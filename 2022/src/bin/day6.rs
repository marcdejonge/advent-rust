#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(map(alpha1, |bs: I| bs.as_bytes().iter().map(|b| b - b'a').collect()))]
struct Input {
    bytes: Vec<u8>,
}

fn calculate_part1(input: &Input) -> u32 {
    input.find(4).expect("Could not find result for part 1")
}

fn calculate_part2(input: &Input) -> u32 {
    input.find(14).expect("Could not find result for part 2")
}

impl Input {
    fn find(&self, size: u32) -> Option<u32> {
        let mut start_iter = self.bytes.iter();
        let mut end_iter = self.bytes.iter();
        let mut mask: u32 = 0;
        let mut count: u32 = 0;

        for _ in 0..size {
            mask ^= 1u32 << end_iter.next()?;
            count += 1;
        }

        while mask.count_ones() != size {
            mask ^= 1u32 << end_iter.next()?;
            mask ^= 1u32 << start_iter.next()?;
            count += 1;
        }

        Some(count)
    }
}

day_main!();
day_test!( 6, example1 => 7, 19 );
day_test!( 6, example2 => 5, 23 );
day_test!( 6, example3 => 6, 23 );
day_test!( 6, example4 => 10, 29 );
day_test!( 6, example5 => 11, 26 );
day_test!( 6 => 1235, 3051 );
