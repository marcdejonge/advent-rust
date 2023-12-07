/*
Something is wrong with global snow production, and you've been selected to take a look. The Elves
have even given you a map; on it, they've used stars to mark the top fifty locations that are likely
to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all
fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're
even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions")
and hang on did you just say the sky ("of course, where do you think snow comes from") when you
realize that the Elves are already loading you into a trebuchet ("please hold still, we need to
strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle
input) has been amended by a very young Elf who was apparently just excited to show off her art
skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a
specific calibration value that the Elves now need to recover. On each line, the calibration value
can be found by combining the first digit and the last digit (in that order) to form a single
two-digit number.

Consider your entire calibration document. What is the sum of all of the calibration values?

--- Part Two ---

Your calculation isn't quite right. It looks like some of the DIGITS are actually spelled out with
letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "DIGITS".

Equipped with this new information, you now need to find the real first and last digit on each line.

What is the sum of all of the calibration values?
*/

#![feature(test)]

use advent_lib::day::*;

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

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { digits: lines.collect() }
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
