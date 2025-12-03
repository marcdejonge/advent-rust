#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(many0(map(one_of("123456789"), |b| b as u8 - b'0')))]
struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn largest_joltage(&self, digits: usize) -> u64 {
        let (mut curr_ix, mut value) = (0, 0);
        for left in (0..digits).rev() {
            let (ix, v) = self.search_largest_joltage(curr_ix, left);
            (curr_ix, value) = (ix + 1, value * 10 + v);
        }
        value
    }

    fn search_largest_joltage(&self, start_ix: usize, left: usize) -> (usize, u64) {
        self.batteries
            .iter()
            .enumerate() // We need the index, such that we can search the next one starting from there
            .skip(start_ix) // Skip any we already had
            .take(self.batteries.len() - left - start_ix) // Also skip the last ones, otherwise we can't search further values
            // HACK: max_by returns the last value, so I'm doing min_by with inverse comparison
            .min_by(|(_, v1), (_, v2)| v2.cmp(v1))
            .map(|(ix, v)| (ix, *v as u64))
            .unwrap()
    }
}

fn calculate_part1(banks: &[Bank]) -> u64 { banks.iter().map(|b| b.largest_joltage(2)).sum() }
fn calculate_part2(banks: &[Bank]) -> u64 { banks.iter().map(|b| b.largest_joltage(12)).sum() }

day_main!(Vec<Bank>);

day_test!( 3, example => 357, 3121910778619 );
day_test!( 3 => 17207, 170997883706617 );
