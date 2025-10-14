#![feature(test)]

use advent_lib::*;
use fxhash::FxHashSet;
use nom_parse_macros::parse_from;

#[parse_from(())]
struct Input {
    rucksacks: Vec<Rucksack>,
}

#[parse_from(map(alpha1, |bs: I| bs.as_bytes().iter().map(|&b| get_priority(b)).collect()))]
#[derive(Clone)]
struct Rucksack(Vec<u32>);

impl Rucksack {
    fn split(&self) -> (FxHashSet<u32>, Vec<u32>) {
        let (left, right) = self.0.split_at(self.0.len() / 2);
        (left.iter().copied().collect(), right.to_vec())
    }

    fn as_set(&self) -> FxHashSet<u32> { self.0.iter().copied().collect() }
}

fn get_priority(b: u8) -> u32 {
    match b {
        b'a'..=b'z' => b as u32 - b'a' as u32 + 1,
        b'A'..=b'Z' => b as u32 - b'A' as u32 + 27,
        _ => unreachable!("Invalid character"),
    }
}

fn calculate_part1(input: &Input) -> u32 {
    input
        .rucksacks
        .iter()
        .map(|line| {
            let (left, right) = line.split();
            right.iter().filter(|&c| left.contains(c)).copied().last().unwrap()
        })
        .sum()
}

fn calculate_part2(input: &Input) -> u32 {
    input
        .rucksacks
        .chunks(3)
        .map(|lines| {
            let first = lines[0].as_set();
            let second = lines[1].as_set();
            let mut third = lines[2].clone().0;
            third.retain(|c| first.contains(c) && second.contains(c));
            third.iter().copied().last().unwrap()
        })
        .sum()
}

day_main!();
day_test!( 3, example => 157, 70 );
day_test!( 3 => 8123, 2620 );
