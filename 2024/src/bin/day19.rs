#![feature(test)]
extern crate core;

use advent_lib::day::*;
use advent_lib::parsing::{full, many_1_n};
use advent_macros::FromRepr;
use nom::bytes::complete::tag;
use nom::multi::{many1, separated_list1};
use rayon::prelude::*;
use smallvec::SmallVec;

#[derive(Debug)]
struct Day {
    nodes: Vec<Node>,
    towels: Vec<Vec<Color>>,
}

#[repr(u8)]
#[derive(FromRepr, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
enum Color {
    White = b'w',
    Blue = b'u',
    Black = b'b',
    Red = b'r',
    Green = b'g',
}

#[derive(Debug)]
struct Node {
    next: [Option<usize>; 5],
    is_end: bool,
}

impl Node {
    fn new() -> Self { Node { next: [None; 5], is_end: false } }

    fn next(&self, color: Color) -> Option<usize> {
        match color {
            Color::White => self.next[0],
            Color::Blue => self.next[1],
            Color::Black => self.next[2],
            Color::Red => self.next[3],
            Color::Green => self.next[4],
        }
    }

    fn next_mut(&mut self, color: Color) -> &mut Option<usize> {
        match color {
            Color::White => self.next.get_mut(0).unwrap(),
            Color::Blue => self.next.get_mut(1).unwrap(),
            Color::Black => self.next.get_mut(2).unwrap(),
            Color::Red => self.next.get_mut(3).unwrap(),
            Color::Green => self.next.get_mut(4).unwrap(),
        }
    }

    fn generate_nodes(available_patterns: Vec<SmallVec<[Color; 8]>>) -> Vec<Node> {
        let mut result = vec![Node::new()];

        for pattern in available_patterns {
            let mut current_ix = 0;
            for color in pattern {
                let next_generated_ix = result.len();
                let current_node = result.get_mut(current_ix).unwrap();
                let next_ix = current_node.next(color);
                if let Some(next_ix) = next_ix {
                    current_ix = next_ix;
                } else {
                    *current_node.next_mut(color) = Some(next_generated_ix);
                    current_ix = next_generated_ix;
                    result.push(Node::new());
                }
            }
            result.get_mut(current_ix).unwrap().is_end = true;
        }

        result
    }
}

impl Day {
    fn can_be_made(&self, towel: &Vec<Color>) -> usize {
        let mut tracker = vec![0usize; self.nodes.len()];
        tracker[0] = 1;

        for &color in towel {
            let mut new_tracker = vec![0; self.nodes.len()];
            for (node_ix, &count) in tracker.iter().enumerate() {
                if count > 0 {
                    let node = self.nodes.get(node_ix).unwrap();
                    if let Some(next_ix) = node.next(color) {
                        new_tracker[next_ix] += count;
                        if self.nodes.get(next_ix).unwrap().is_end {
                            new_tracker[0] += count;
                        }
                    }
                }
            }
            tracker = new_tracker;
        }

        tracker[0]
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let first_line = lines.next().expect("First line with available patterns");
        let available_patterns =
            full(separated_list1(tag(", "), many_1_n(Color::parse)))(&first_line).unwrap();
        lines.next().expect("Empty line after available patterns");
        let towels = lines.filter_map(|s| full(many1(Color::parse))(&s).ok()).collect();

        Day { nodes: Node::generate_nodes(available_patterns), towels }
    }
    fn calculate_part1(&self) -> Self::Output {
        self.towels.par_iter().filter(|&towel| self.can_be_made(towel) > 0).count()
    }
    fn calculate_part2(&self) -> Self::Output {
        self.towels.par_iter().map(|towel| self.can_be_made(towel)).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 19, example1 => 6, 16 );
    day_test!( 19 => 272, 1041529704688380 );
}
