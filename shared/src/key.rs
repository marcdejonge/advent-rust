use crate::parsing::Parsable;
use nom::bytes::complete::take_while_m_n;
use nom::character::is_alphabetic;
use nom::combinator::map;
use nom::error::Error;
use nom::Parser;
use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Key {
    value: u64,
}

impl Key {
    /// This const function is to be able to create const Keys. It contains fewer checks that the
    /// from_str, so it shouldn't be used in normal parsing.
    pub const fn fixed(s: &'static [u8]) -> Key {
        let mut value: u64 = 0;
        let mut ix = 0;
        while ix < s.len() {
            let c = if s[ix].is_ascii_lowercase() {
                s[ix] - b'a'
            } else if s[ix].is_ascii_uppercase() {
                s[ix] - b'A'
            } else {
                panic!("Invalid character found")
            };
            value = value * 26 + c as u64 + 1;
            ix += 1;
        }
        value -= 1;
        Key { value }
    }

    pub const fn last_char(&self) -> u8 { b'a' + (self.value % 26) as u8 }
}

const LOW_OFFSET: u64 = b'a' as u64;
const HIGH_OFFSET: u64 = b'A' as u64;

impl Parsable for Key {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(take_while_m_n(1, 12, is_alphabetic), |key: &[u8]| {
            let value = key.iter().fold(0, |acc, &c| {
                let codepoint = if c.is_ascii_lowercase() {
                    c as u64 - LOW_OFFSET
                } else if c.is_ascii_uppercase() {
                    c as u64 - HIGH_OFFSET
                } else {
                    unreachable!("Invalid character found");
                };

                acc * 26 + codepoint + 1
            }) - 1;

            Key { value }
        })
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Key(")?;
        Display::fmt(self, f)?;
        f.write_str(")")?;
        Ok(())
    }
}

const ADD_OFFSET: u32 = 'a' as u32;

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn recurse(mut value: u64, f: &mut Formatter) -> std::fmt::Result {
            if value == 0 {
                return Ok(());
            }

            value -= 1;
            let c =
                char::try_from((value % 26) as u32 + ADD_OFFSET).map_err(|_| std::fmt::Error)?;
            recurse(value / 26, f)?;
            f.write_char(c)
        }

        recurse(self.value + 1, f)
    }
}

/// This is to generate readable keys from indices used in an array or vector
impl From<usize> for Key {
    fn from(value: usize) -> Self { Key { value: value as u64 } }
}

/// This is the reverse, to be able to use a key as an index in an array or vector
impl From<Key> for usize {
    fn from(key: Key) -> Self { key.value as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_and_forth() {
        for test_string in [&b"aaa"[..], &b"zzz"[..], &b"hello"[..], &b"world"[..]] {
            let key = match Key::parser().parse(test_string) {
                Ok((_, x)) => x,
                Err(_) => panic!("Could not parse"),
            };

            assert_eq!(test_string, key.to_string().as_bytes())
        }
    }

    #[test]
    fn stop_parsing_on_other_characters() {
        assert_eq!(
            Ok((&b"7"[..], Key::fixed(b"abc"))),
            Key::parser().parse(b"abc7")
        );
    }

    #[test]
    fn parse_maximum_of_12_characters() {
        assert_eq!(
            Ok((&b"mno"[..], Key::fixed(b"abcdefghijkl"))),
            Key::parser().parse(b"abcdefghijklmno")
        );
    }

    #[test]
    fn predictable_raw_keys() {
        assert_eq!(Key::parser().parse(b"a").unwrap().1, Key::from(0));
        assert_eq!(Key::parser().parse(b"ab").unwrap().1, Key::from(27));
        assert_eq!(
            Key::parser().parse(b"columns").unwrap().1,
            Key::from(1110829946)
        );
    }

    #[test]
    fn const_keys_should_match() {
        assert_eq!(Key::parser().parse(b"text").unwrap().1, Key::fixed(b"text"));
    }
}
