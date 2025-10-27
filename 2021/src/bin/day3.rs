#![feature(test, int_from_ascii)]

use advent_lib::{day_main, day_test};
use nom::{
    bytes::complete::is_a, character::complete::line_ending, combinator::map,
    multi::separated_list0, Parser,
};
use nom_parse_trait::ParseFrom;

struct Input {
    numbers: Vec<u32>,
    all_bits: Vec<u32>,
}

impl<'a> ParseFrom<&'a [u8]> for Input {
    fn parse(input: &'a [u8]) -> nom::IResult<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        map(
            separated_list0(
                line_ending,
                map(is_a("01"), |s: &'a [u8]| {
                    s.iter()
                        .fold(0u32, |acc, &b| (acc << 1) | ((b - b'0') as u32))
                }),
            ),
            |numbers| {
                let bit_count = numbers.iter().fold(0u32, |acc, nr| acc | nr).count_ones();
                let all_bits = (0..bit_count).map(|i| 1 << i).collect();
                Input { numbers, all_bits }
            },
        )
        .parse_complete(input)
    }
}

fn has_majority_ones(numbers: &Vec<u32>, mask: u32) -> bool {
    numbers.iter().filter(|&&nr| (nr & mask) != 0).count() * 2 >= numbers.len()
}

impl Input {
    fn find_rating(&self, majority: bool) -> u32 {
        let mut candidates = self.numbers.clone();
        for &mask in self.all_bits.iter().rev() {
            let desired_bit = has_majority_ones(&candidates, mask) ^ !majority;
            candidates.retain(|&nr| ((nr & mask) != 0) == desired_bit);
            if candidates.len() == 1 {
                break;
            }
        }
        candidates[0]
    }
}

fn calculate_part1(input: &Input) -> u32 {
    let (gamma, epsilon) = input
        .all_bits
        .iter()
        .fold((0u32, 0u32), |(gamma, epsilon), &mask| {
            if has_majority_ones(&input.numbers, mask) {
                (gamma | mask, epsilon)
            } else {
                (gamma, epsilon | mask)
            }
        });
    gamma * epsilon
}

fn calculate_part2(input: &Input) -> u32 {
    let oxygen = input.find_rating(true);
    let co2 = input.find_rating(false);
    oxygen * co2
}

day_main!(calculate_part1, calculate_part2);

day_test!( 3, example => 198, 230 );
day_test!( 3 => 2261546, 6775520 );
