use crate::parsing::Parsable;
use nom::bytes::complete::take_while_m_n;
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::error::Error;
use nom::Parser;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Add;
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Key {
    value: u64,
}

impl Key {
    /// This const function is to be able to create const Keys. It contains fewer checks that the
    /// from_str, so it shouldn't be used in normal parsing.
    pub const fn fixed(s: &'static [u8]) -> Key {
        if s.len() > 12 {
            panic!("Key is too long")
        }

        let mut value: u64 = 0;
        let mut ix = 0;
        while ix < s.len() {
            if s[ix].is_ascii_lowercase() {
                value = value * 36 + (s[ix] - b'a') as u64 + 1;
                ix += 1;
            } else if s[ix].is_ascii_uppercase() {
                value = value * 36 + (s[ix] - b'A') as u64 + 1;
                ix += 1;
            } else if s[ix].is_ascii_digit() {
                value = value * 36 + (s[ix] - b'0') as u64 + 27;
                ix += 1;
            } else {
                panic!("Invalid character found")
            }
        }
        value -= 1;
        Key { value }
    }

    pub const fn last_char(&self) -> u8 { b'a' + (self.value % 36) as u8 }
}

const LOW_OFFSET: u64 = b'a' as u64;
const HIGH_OFFSET: u64 = b'A' as u64;
const DIGIT_OFFSET: u64 = b'0' as u64;

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match Key::parser().parse(s.as_bytes()) {
            Ok((_, x)) => x,
            Err(_) => return Err(()),
        };

        Ok(key)
    }
}

impl Parsable for Key {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(take_while_m_n(1, 12, is_alphanumeric), |key: &[u8]| {
            let value = key.iter().fold(0, |acc, &c| {
                if c.is_ascii_lowercase() {
                    acc * 36 + (c as u64 - LOW_OFFSET) + 1
                } else if c.is_ascii_uppercase() {
                    acc * 36 + (c as u64 - HIGH_OFFSET) + 1
                } else if c.is_ascii_digit() {
                    acc * 36 + (c as u64 - DIGIT_OFFSET) + 27
                } else {
                    unreachable!("Invalid character found");
                }
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

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn recurse(mut value: u64, f: &mut Formatter) -> std::fmt::Result {
            if value == 0 {
                return Ok(());
            }

            value -= 1;
            let code_point = (value % 36) as u32;
            let c = if code_point >= 26 {
                char::try_from(code_point - 26 + DIGIT_OFFSET as u32)
                    .map_err(|_| std::fmt::Error)?
            } else {
                char::try_from(code_point + LOW_OFFSET as u32).map_err(|_| std::fmt::Error)?
            };
            recurse(value / 36, f)?;
            f.write_char(c)
        }

        recurse(self.value + 1, f)
    }
}

impl Add<usize> for Key {
    type Output = Key;

    fn add(self, rhs: usize) -> Self::Output { Key { value: self.value + rhs as u64 } }
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
            Ok((&b"*"[..], Key::fixed(b"abc7"))),
            Key::parser().parse(b"abc7*")
        );
    }

    #[test]
    fn parse_maximum_of_12_characters() {
        assert_eq!(
            Ok((&b"klmno"[..], Key::fixed(b"abcdefghij"))),
            Key::parser().parse(b"abcdefghijklmno")
        );
    }

    #[test]
    fn predictable_raw_keys() {
        assert_eq!(Key::parser().parse(b"a").unwrap().1, Key::from(0));
        assert_eq!(Key::parser().parse(b"ab").unwrap().1, Key::from(37));
        assert_eq!(
            Key::parser().parse(b"columns").unwrap().1,
            Key::from(7458492186)
        );
    }

    #[test]
    fn const_keys_should_match() {
        assert_eq!(Key::parser().parse(b"text").unwrap().1, Key::fixed(b"text"));
    }
}
