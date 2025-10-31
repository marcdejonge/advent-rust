#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(separated_list1(line_ending, map(alphanumeric1, |bs: I| bs.as_bytes().to_vec())))]
struct Input {
    digits: Vec<Vec<u8>>,
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

fn calculate_part1(input: &Input) -> u32 {
    input.digits.iter().map(|line| parse_line(line, false)).sum()
}

fn calculate_part2(input: &Input) -> u32 {
    input.digits.iter().map(|line| parse_line(line, true)).sum()
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

day_main!(Input);
day_test!( 1, example1 => 142, 142 );
day_test!( 1, example2 => 209, 281 );
day_test!( 1 => 54338, 53389);
