#![feature(test)]

use advent_lib::*;
use nom::{
    Parser,
    character::complete::{space0, space1},
    multi::separated_list1,
    sequence::delimited,
};
use nom_parse_macros::parse_from;

// I hate how I did this, but I can't be bothered on a better solution...

#[parse_from(separated_pair(
    separated_list1(newline, fold_many1(one_of("123456789 "), Vec::new, |mut accum, next| {
        accum.push(next as u8);
        accum
    })),
    newline,
    many1((one_of("*+"), many1_count(one_of(" \n"))))
))]
struct Input {
    lines: Vec<Vec<u8>>,
    operations: Vec<(char, usize)>,
}

fn calculate_part1(input: &Input) -> u64 {
    let numbers: Vec<_> = input
        .lines
        .iter()
        .map(|line| {
            delimited(
                space0,
                separated_list1::<_, (), _, _>(space1, nom::character::complete::u64),
                space0,
            )
            .parse_complete(line.as_slice())
            .unwrap()
            .1
        })
        .collect();

    input
        .operations
        .iter()
        .enumerate()
        .map(|(ix, (op, _))| {
            if *op == '+' {
                numbers.iter().map(|nrs| nrs[ix]).sum::<u64>()
            } else {
                // '*'
                numbers.iter().map(|nrs| nrs[ix]).product()
            }
        })
        .sum()
}

fn calculate_part2(input: &Input) -> u64 {
    let mut ix = 0;
    input
        .operations
        .iter()
        .map(|(op, cnt)| {
            let mut accum = if *op == '+' { 0u64 } else { 1 };
            for _ in 0..*cnt {
                let nr = input.lines.iter().fold(0u64, |accum, line| {
                    let c = line.get(ix).unwrap_or(&b' ');
                    match c {
                        b' ' => accum,
                        b'1'..=b'9' => accum * 10 + (c - b'0') as u64,
                        _ => unreachable!(),
                    }
                });
                ix += 1;
                if *op == '+' {
                    accum += nr;
                } else {
                    accum *= nr;
                }
            }
            ix += 1;
            accum
        })
        .sum()
}

day_main!(Input);

day_test!( 6, example => 4277556, 3263827 );
day_test!( 6 => 5552221122013, 11371597126232 );
