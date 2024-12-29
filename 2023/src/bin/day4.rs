#![feature(test)]

use advent_lib::day_main;
use advent_macros::parsable;
use fxhash::FxHashSet;
use std::ops::Shl;

#[parsable(separated_lines1())]
struct Input {
    cards: Vec<Card>,
}

#[parsable(
    preceded(
        tuple((tag(b"Card"), space1, u64, tag(b":"), space1)),
        separated_pair(
            map(separated_list1(space1, u8), |winning| winning.into_iter().collect()),
            tuple((tag(b" |"), space1)),
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
        for next_ix in ix + 1..=ix + c.winning_count() {
            counts[next_ix] += curr_count;
        }
    });
    counts.iter().sum()
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example => 13, 30 );
    day_test!( 4 => 18519, 11787590);
}
