/*
--- Day 4: Scratchcards ---

The gondola takes you up. Strangely, though, the ground doesn't seem to be coming with you; you're
not climbing a mountain. As the circle of Snow Island recedes below you, an entire new landmass
suddenly appears above you! The gondola carries you to the surface of the new island and lurches
into the station.

As you exit the gondola, the first thing you notice is that the air here is much warmer than it was
on Snow Island. It's also quite humid. Is this where the water source is?

The next thing you notice is an Elf sitting on the floor across the station in what seems to be a
pile of colorful square cards.

"Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You ask about water
sources.

"I'm not sure; I just operate the gondola lift. That does sound like something we'd have, though -
this is Island Island, after all! I bet the gardener would know. He's on a different island, though
- er, the small kind surrounded by water, not the floating kind. We really need to come up with a
better naming scheme. Tell you what: if you can help me with something quick, I'll let you borrow my
boat and you can go visit the gardener. I got all these scratchcards as a gift, but I can't figure
out what I've won."

The Elf leads you over to the pile of colorful cards. There, you discover dozens of scratchcards,
all with their opaque covering already scratched off. Picking one up, it looks like each card has
two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of
numbers you have. You organize the information into a table (your puzzle input).

As far as the Elf has been able to figure out, you have to figure out which of the numbers you have
appear in the list of winning numbers. The first match makes the card worth one point and each match
after the first doubles the point value of that card.

Take a seat in the large pile of colorful cards. How many points are they worth in total?

--- Part Two ---

Just as you're about to report your findings to the Elf, one of you realizes that the rules have
actually been printed on the back of every card this whole time.

There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards
equal to the number of winning numbers you have.

Specifically, you win copies of the scratchcards below the winning card equal to the number of
matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11,
12, 13, 14, and 15.

Copies of scratchcards are scored like normal scratchcards and have the same card number as the
card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win
a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process
repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a
card past the end of the table.)

Process all of the original and copied scratchcards until no more scratchcards are won. Including
the original set of scratchcards, how many total scratchcards do you end up with?
 */

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
