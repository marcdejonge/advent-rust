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
            let c = s[ix] - b'a' + 1;
            value = value * 27 + c as u64;
        }
        Key { value }
    }
}

const OFFSET: u32 = 'a' as u32 - 1;

impl<'a> Parse<'a> for Key {
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut value: u64 = 0;
        for c in s.chars().rev() {
            if !c.is_ascii_lowercase() {
                return Err(ParseError::new("Invalid character found"));
            }

            value = value.checked_mul(27).ok_or(ParseError::new("Overflow error"))?
                + (c as u32 - OFFSET) as u64;
        }

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

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut value = self.value;

        while value > 0 {
            f.write_char(
                char::try_from((value % 27) as u32 + OFFSET).map_err(|_| std::fmt::Error)?,
            )?;
            value /= 27;
        }

        Ok(())
    }
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
            Key::from_str("abcdefghijklmn")
        );
    }
}
