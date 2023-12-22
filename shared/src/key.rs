use std::fmt::{Debug, Display, Formatter, Write};

use prse::{Parse, ParseError};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Key {
    value: u64,
}

impl Key {
    /// This const function is to be able to create const Keys. It contains less checks that the
    /// from_str, so it shouldn't be used in normal parsing.
    pub const fn fixed(s: &'static [u8]) -> Key {
        let mut value: u64 = 0;
        let mut ix = s.len();
        while ix > 0 {
            ix -= 1;
            let c = if s[ix].is_ascii_lowercase() {
                s[ix] - b'a'
            } else if s[ix].is_ascii_uppercase() {
                s[ix] - b'A'
            } else {
                panic!("Invalid character found")
            };
            value = value * 26 + c as u64 + 1;
        }
        value -= 1;
        Key { value }
    }

    pub const fn last_char(&self) -> u8 { b'a' + (self.value % 26) as u8 }
}

const LOW_OFFSET: u64 = 'a' as u64;
const HIGH_OFFSET: u64 = 'A' as u64;

impl<'a> Parse<'a> for Key {
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if s.is_empty() {
            ParseError::new("Empty strings are not supported");
        }

        let mut value: u64 = 0;
        for c in s.chars() {
            let codepoint = if c.is_ascii_lowercase() {
                c as u64 - LOW_OFFSET
            } else if c.is_ascii_uppercase() {
                c as u64 - HIGH_OFFSET
            } else {
                return Err(ParseError::new("Invalid character found"));
            };

            value = value
                .checked_mul(26)
                .ok_or(ParseError::new("Overflow error"))?
                .checked_add(codepoint)
                .ok_or(ParseError::new("Overflow error"))?
                + 1;
        }
        value -= 1;

        Ok(Key { value })
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
        for test_string in ["aaa", "zzz", "hello", "world"] {
            let key = match Key::from_str(test_string) {
                Ok(x) => x,
                Err(_) => panic!("Could not parse"),
            };

            assert_eq!(test_string, &key.to_string())
        }
    }

    #[test]
    fn other_characters_should_fail() {
        assert_eq!(
            Err(ParseError::Other("Invalid character found".to_string())),
            Key::from_str("abc7")
        );
    }

    #[test]
    fn string_too_long_error() {
        assert_eq!(
            Err(ParseError::Other("Overflow error".to_string())),
            Key::from_str("abcdefghijklmno")
        );
    }

    #[test]
    fn predictable_raw_keys() {
        assert_eq!(Key::from_str("a"), Ok(Key::from(0)));
        assert_eq!(Key::from_str("ab"), Ok(Key::from(27)));
        assert_eq!(Key::from_str("columns"), Ok(Key::from(1110829946)));
    }
}
