#![feature(test)]

use advent_lib::day::*;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Parser;

struct Day {
    digits: Vec<String>,
}

const DIGITS: &[(&[u8], u32)] = &[
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

impl ExecutableDay for Day {
    type Output = u32;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(line_ending, alphanumeric1),
            |digits: Vec<&[u8]>| Day {
                digits: digits.iter().map(|bs| String::from_utf8_lossy(bs).to_string()).collect(),
            },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        self.digits.iter().map(|line| parse_line(line.as_bytes(), false)).sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.digits.iter().map(|line| parse_line(line.as_bytes(), true)).sum()
    }
}

fn parse_line(line: &[u8], check_text: bool) -> u32 {
    let mut first = 0;
    for ix in 0..line.len() {
        if let Some(result) = parse_prefix(&line[ix..], check_text) {
            first = result;
            break;
        }
    }

    let mut last = 0;
    for ix in (0..line.len()).rev() {
        if let Some(result) = parse_prefix(&line[ix..], check_text) {
            last = result;
            break;
        }
    }

    first * 10 + last
}

fn parse_prefix(line: &[u8], check_text: bool) -> Option<u32> {
    if line.is_empty() {
        return None;
    } else if (b'1'..=b'9').contains(&line[0]) {
        return Some((line[0] - b'0') as u32);
    } else if check_text {
        for (test, result) in DIGITS {
            if line.starts_with(test) {
                return Some(*result);
            }
        }
    }

    None
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example1 => 142, 142 );
    day_test!( 1, example2 => 209, 281 );
    day_test!( 1 => 54338, 53389);
}
