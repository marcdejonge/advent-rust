#![feature(test)]

use advent_lib::parsing::separated_lines1;
use advent_lib::*;
use fxhash::FxHashSet;
use nom_parse_macros::parse_from;
use std::ops::Shl;

#[parse_from(separated_lines1())]
struct Input {
    cards: Vec<Card>,
}

#[parse_from(
    preceded(
        ("Card", space1, u64, ":", space1),
        separated_pair(
            map(separated_list1(space1, u8), |winning| winning.into_iter().collect()),
            (" |", space1),
            separated_list1(space1, u8),
        ),
    )
)]
struct Card {
    winning: FxHashSet<u8>,
    drawn: Vec<u8>,
}

impl Card {
    fn winning_count(&self) -> usize {
        self.drawn.iter().filter(|n| self.winning.contains(n)).count()
    }
}

fn calculate_part1(input: &Input) -> usize {
    input
        .cards
        .iter()
        .map(|c| {
            let count = c.winning_count();
            if count == 0 {
                0
            } else {
                1usize.shl(count - 1)
            }
        })
        .sum()
}

fn calculate_part2(input: &Input) -> usize {
    let mut counts = Vec::with_capacity(input.cards.len());
    for _ in 0..input.cards.len() {
        counts.push(1usize)
    }

    input.cards.iter().enumerate().for_each(|(ix, c)| {
        let curr_count = counts[ix];

        counts.as_mut_slice()[ix + 1..=ix + c.winning_count()]
            .iter_mut()
            .for_each(|c| *c += curr_count);
    });
    counts.iter().sum()
}

day_main!();
day_test!( 4, example => 13, 30 );
day_test!( 4 => 18519, 11787590);
