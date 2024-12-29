#![feature(test)]

use advent_lib::day_main;
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::key::Key;
use advent_macros::parsable;
use fxhash::FxHashMap;
use num::integer::lcm;
use rayon::prelude::*;

#[parsable(
    separated_pair(
        many1(alt((
            map(tag(b"L"), |_| Turn::Left),
            map(tag(b"R"), |_| Turn::Right),
        ))),
        double_line_ending,
        map(
            separated_list1(
                line_ending,
                separated_pair(
                    Key::parser(),
                    tag(b" = "),
                    delimited(tag(b"("),
                        separated_pair(Key::parser(), tag(b", "), Key::parser()),
                    tag(b")")),
                ),
            ),
            |list| list.into_iter().collect()
        )
    )
)]
struct Input {
    instructions: Vec<Turn>,
    steps: FxHashMap<Key, (Key, Key)>,
}

enum Turn {
    Left,
    Right,
}

impl Input {
    fn next_step(&self, curr: Key, turn: &Turn) -> Option<Key> {
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

    fn walk(&self, start: Key) -> impl Iterator<Item = Key> + '_ {
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

fn calculate_part1(input: &Input) -> usize {
    const TARGET: Key = Key::fixed(b"zzz");
    input.walk(Key::fixed(b"aaa")).take_while(|&c| c != TARGET).count() + 1
}

fn calculate_part2(input: &Input) -> usize {
    let end_steps: Vec<_> = input
        .steps
        .par_iter()
        .filter(|(p, _)| p.last_char() == b'a')
        .map(|(start, _)| {
            for (step, place) in input.walk(*start).enumerate() {
                if place.last_char() == b'z' {
                    return step + 1;
                }
            }
            0
        })
        .collect();

    end_steps.iter().fold(1, |curr, next| lcm(curr, *next))
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 8, example1 => 2, 2);
    day_test!( 8, example2 => 6, 6);
    day_test!( 8, example3 => 1, 6);
    day_test!( 8 => 12361, 18215611419223);
}
