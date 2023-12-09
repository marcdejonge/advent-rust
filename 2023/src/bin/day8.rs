/*
--- Day 8: Haunted Wasteland ---

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching.
When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished
warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle
input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of
the documents contains a list of left/right instructions, and the rest of the documents seem to
describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if
you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where
you are now, and you have to follow the left/right instructions until you reach ZZZ.

Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?

--- Part Two ---

The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel
follow the instructions, but you've barely left your starting position. It's going to take
significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the
laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of
nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably
just start at every node that ends with A and follow all of the paths at the same time until they
all simultaneously end up at nodes that end with Z.

Simultaneously start on every node that ends with A. How many steps does it take before you're only
on nodes that end with Z?
*/

#![feature(test)]

use fxhash::FxHashMap;
use num::integer::lcm;
use prse_derive::parse;
use rayon::prelude::*;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::iter_utils::RepeatingIteratorTrait;

type Place = [u8; 3];

struct Day {
    instructions: Vec<Turn>,
    steps: FxHashMap<Place, (Place, Place)>,
}

enum Turn {
    Left,
    Right,
}

const fn parse_code(s: &str) -> Place {
    let bytes = s.as_bytes();
    [bytes[0], bytes[1], bytes[2]]
}

impl Day {
    fn next_step(&self, curr: Place, turn: &Turn) -> Option<Place> {
        let next_steps = self.steps.get(&curr);
        let next = if let Some((left, right)) = next_steps {
            match turn {
                Turn::Left => *left,
                Turn::Right => *right,
            }
        } else {
            return None;
        };
        if curr == next {
            None
        } else {
            Some(next)
        }
    }

    fn walk<'a>(&'a self, start: Place) -> impl Iterator<Item = Place> + 'a {
        self.instructions.iter().repeat().scan(start, |curr, turn| {
            if let Some(next) = self.next_step(*curr, turn) {
                *curr = next;
                Some(next)
            } else {
                None
            }
        })
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Turn::Left,
                'R' => Turn::Right,
                _ => panic!("Unknown turn {c}"),
            })
            .collect();
        lines.next().unwrap(); // Empty line

        let mut steps = FxHashMap::default();
        lines.for_each(|line| {
            let (from, left, right) = parse!(line, "{} = ({}, {})");
            steps.insert(parse_code(from), (parse_code(left), parse_code(right)));
        });

        Day { instructions, steps }
    }

    fn calculate_part1(&self) -> Self::Output {
        let target = parse_code("ZZZ");
        self.walk(parse_code("AAA")).take_while(|c| c != &target).count() + 1
    }

    fn calculate_part2(&self) -> Self::Output {
        let end_steps: Vec<_> = self
            .steps
            .par_iter()
            .filter(|(p, _)| p[2] == b'A')
            .map(|(start, _)| {
                for (step, place) in self.walk(*start).enumerate() {
                    if place[2] == b'Z' {
                        return step + 1;
                    }
                }
                0
            })
            .collect();

        end_steps.iter().fold(1, |curr, next| lcm(curr, *next))
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 8, example1 => 2, 2);
    day_test!( 8, example2 => 6, 6);
    day_test!( 8, example3 => 1, 6);
    day_test!( 8 => 12361, 18215611419223);
}
