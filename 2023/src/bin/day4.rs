#![feature(test)]

use fxhash::FxHashSet;
use std::ops::Shl;
use std::str::FromStr;

use advent_lib::day::*;

struct Day {
    cards: Vec<Card>,
}

struct Card {
    winning: FxHashSet<u8>,
    drawn: Vec<u8>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, nrs) = s.split_at(s.find(':').ok_or("Could not find colon")? + 2);
        let (winning, drawn) = nrs.split_at(nrs.find('|').ok_or("Could not find pipe")?);
        Ok(Card {
            winning: winning.split(' ').filter_map(|n| n.parse().ok()).collect(),
            drawn: drawn.split(' ').filter_map(|n| n.parse().ok()).collect(),
        })
    }
}

impl Card {
    fn winning_count(&self) -> usize {
        self.drawn.iter().filter(|n| self.winning.contains(n)).count()
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { cards: lines.map(|line| line.parse().unwrap()).collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.cards
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

    fn calculate_part2(&self) -> Self::Output {
        let mut counts = Vec::with_capacity(self.cards.len());
        for _ in 0..self.cards.len() {
            counts.push(1usize)
        }

        self.cards.iter().enumerate().for_each(|(ix, c)| {
            let curr_count = counts[ix];
            for next_ix in ix + 1..=ix + c.winning_count() {
                counts[next_ix] += curr_count;
            }
        });
        counts.iter().sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example => 13, 30 );
    day_test!( 4 => 18519, 11787590);
}
