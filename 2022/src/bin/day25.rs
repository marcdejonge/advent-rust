#![feature(test)]

use advent_lib::{day_main_half, day_test};
use nom::bytes::take_while;
use nom::combinator::map;
use nom::{IResult, Parser};
use nom_parse_trait::ParseFrom;
use std::fmt::Display;

struct SnafuNumber(u64);

impl ParseFrom<&[u8]> for SnafuNumber {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        map(take_while(|b| b"210-=".contains(&b)), |bytes: &[u8]| {
            SnafuNumber(bytes.iter().fold(0u64, |curr, next| match next {
                b'2' => curr * 5 + 2,
                b'1' => curr * 5 + 1,
                b'0' => curr * 5,
                b'-' => curr * 5 - 1,
                b'=' => curr * 5 - 2,
                _ => unreachable!(),
            }))
        })
        .parse_complete(input)
    }
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = [b'0'; 28]; // 28 digits are enough to represent u64 in base 5 with snafu encoding
        let mut index = buffer.len(); // start from the end of the buffer and write backwards
        let mut value = self.0;
        while index > 0 && value != 0 {
            let remainder = value % 5;
            value /= 5;
            index -= 1;
            match remainder {
                0 => buffer[index] = b'0',
                1 => buffer[index] = b'1',
                2 => buffer[index] = b'2',
                3 => {
                    buffer[index] = b'=';
                    value += 1;
                }
                4 => {
                    buffer[index] = b'-';
                    value += 1;
                }
                _ => unreachable!(),
            }
        }
        // SAFETY: We've only generates ASCII bytes
        f.write_str(unsafe { str::from_utf8_unchecked(&buffer[index..]) })
    }
}

fn calculate_part1(numbers: &[SnafuNumber]) -> String {
    SnafuNumber(numbers.iter().fold(0, |curr, next| curr + next.0)).to_string()
}

day_main_half!(Vec<SnafuNumber>);

day_test!( 25, example => "2=-1=0".to_string() );
day_test!( 25 => "2-2=12=1-=-1=000=222".to_string() );
