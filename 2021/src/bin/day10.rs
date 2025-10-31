#![feature(test)]

use advent_lib::{builder::with_default, *};

#[derive(Default)]
struct MatchingPair {
    stack: Vec<u8>,
    corruption_char: Option<u8>,
}

impl nom_parse_trait::ParseFrom<&[u8]> for MatchingPair {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        use nom::Parser;
        nom::combinator::map(nom::bytes::complete::is_a("()<>{}[]"), |bs: &[u8]| {
            with_default(|it: &mut MatchingPair| {
                for b in bs {
                    if !it.add_char(*b) {
                        break;
                    }
                }
            })
        })
        .parse(input)
    }
}

impl MatchingPair {
    fn pop_matching_char(&mut self, next: u8, expected: u8) -> bool {
        if self.stack.pop() == Some(expected) {
            true
        } else {
            self.corruption_char = Some(next);
            false
        }
    }

    fn add_char(&mut self, next: u8) -> bool {
        self.corruption_char.is_none()
            && match next {
                b'(' | b'[' | b'{' | b'<' => {
                    self.stack.push(next);
                    true
                }
                b')' => self.pop_matching_char(next, b'('),
                b'>' => self.pop_matching_char(next, b'<'),
                b']' => self.pop_matching_char(next, b'['),
                b'}' => self.pop_matching_char(next, b'{'),
                _ => unreachable!(),
            }
    }
}

fn calculate_part1(input: &[MatchingPair]) -> u64 {
    input
        .iter()
        .flat_map(|mp| mp.corruption_char)
        .map(|b| match b {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => 0,
        })
        .sum()
}

fn calculate_part2(input: &[MatchingPair]) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter(|mp| mp.corruption_char.is_none())
        .map(|mp| {
            mp.stack.iter().rev().fold(0, |curr, next| {
                5 * dbg!(curr)
                    + match next {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => 0,
                    }
            })
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

day_main!(Vec<MatchingPair>);

day_test!( 10, example => 26397, 288957 );
day_test!( 10 => 436497, 2377613374 );
