#![feature(test)]

use advent_lib::day::*;
use advent_macros::parsable;

#[parsable(separated_list1(line_ending, map(alphanumeric1, |bs: &[u8]| bs.to_vec())))]
struct Day {
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

impl ExecutableDay for Day {
    type Output = u32;

    fn calculate_part1(&self) -> u32 {
        self.digits.iter().map(|line| parse_line(line, false)).sum()
    }

    fn calculate_part2(&self) -> u32 { self.digits.iter().map(|line| parse_line(line, true)).sum() }
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
