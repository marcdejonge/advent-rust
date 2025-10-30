#![feature(test)]

use advent_lib::{iter_utils::IteratorUtils, *};
use fxhash::FxHashMap;
use nom::*;
use nom_parse_macros::parse_from;
use std::ops::Shl;

#[derive(Debug)]
#[parse_from(separated_pair(parse_wires(), " | ", parse_wires()))]
struct Line {
    wires: Vec<u32>,
    digits: Vec<u32>,
}

fn parse_wires<I: Input, E: error::ParseError<I>>() -> impl Parser<I, Output = Vec<u32>, Error = E>
where
    <I as Input>::Item: AsChar,
{
    multi::separated_list1(
        character::complete::space1,
        combinator::map(
            bytes::complete::take_while1(AsChar::is_alpha),
            |input: I| {
                input.iter_elements().fold(0u32, |curr, nr: I::Item| {
                    1u32.shl(nr.as_char() as u32 - 'a' as u32) | curr
                })
            },
        ),
    )
}

struct DigitFinder<'a>(&'a Vec<u32>);

impl<'a> DigitFinder<'a> {
    fn find_digit(&self, segments: u32) -> u32 {
        self.find_digit_filtered(segments, |_| true)
    }

    fn find_digit_filtered<F>(&self, segments: u32, filter: F) -> u32
    where
        F: Fn(u32) -> bool,
    {
        self.0
            .iter()
            .copied()
            .filter(|nr| nr.count_ones() == segments)
            .filter(|&nr| filter(nr))
            .single()
            .unwrap()
    }
}

fn detect_digits(wires: &Vec<u32>) -> FxHashMap<u32, u32> {
    let finder = DigitFinder(wires);
    let n1 = finder.find_digit(2);
    let n7 = finder.find_digit(3);
    let n4 = finder.find_digit(4);
    let n8 = finder.find_digit(7);
    let n9 = finder.find_digit_filtered(6, |d| (d & n4) == n4);
    let n0 = finder.find_digit_filtered(6, |d| d != n9 && (d & n1) == n1);
    let n6 = finder.find_digit_filtered(6, |d| d != n9 && d != n0);
    let n3 = finder.find_digit_filtered(5, |d| (d & n1) == n1);
    let n5 = finder.find_digit_filtered(5, |d| d != n3 && (d & n9) == d);
    let n2 = finder.find_digit_filtered(5, |d| d != n3 && d != n5);
    [
        (n0, 0),
        (n1, 1),
        (n2, 2),
        (n3, 3),
        (n4, 4),
        (n5, 5),
        (n6, 6),
        (n7, 7),
        (n8, 8),
        (n9, 9),
    ]
    .into_iter()
    .collect()
}

#[allow(clippy::ptr_arg)]
fn calculate_part1(input: &Vec<Line>) -> usize {
    input
        .iter()
        .map(|Line { wires: _, digits }| {
            digits
                .iter()
                .filter(|nr| [2, 3, 4, 7].contains(&nr.count_ones()))
                .count()
        })
        .sum()
}

#[allow(clippy::ptr_arg)]
fn calculate_part2(input: &Vec<Line>) -> u64 {
    input
        .iter()
        .map(|Line { wires, digits }| {
            let mapping = detect_digits(wires);
            digits
                .iter()
                .map(|nr| mapping.get(nr).unwrap())
                .fold(0, |p, d| p * 10 + d) as u64
        })
        .sum()
}

day_main!();

day_test!( 8, example => 26, 61229 );
day_test!( 8 => 479, 1041746 );
