#![feature(test)]

use fxhash::FxHashMap;
use num::integer::lcm;
use prse_derive::parse;
use rayon::prelude::*;

use advent_lib::day::*;
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

    fn walk(&self, start: Place) -> impl Iterator<Item = Place> + '_ {
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
