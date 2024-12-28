use crate::parsing::Parsable;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::preceded;
use nom::Parser;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Parsable for RGB {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map_res(
            preceded(
                tag(b"#"),
                take_while_m_n(6, 6, |b: u8| b.is_ascii_hexdigit()),
            ),
            |digits: &'a [u8]| {
                let red = u8::from_str_radix(std::str::from_utf8(&digits[0..2]).unwrap(), 16)?;
                let green = u8::from_str_radix(std::str::from_utf8(&digits[2..4]).unwrap(), 16)?;
                let blue = u8::from_str_radix(std::str::from_utf8(&digits[4..6]).unwrap(), 16)?;
                Ok::<RGB, ParseIntError>(RGB { red, green, blue })
            },
        )
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "#{:x}{:x}{:x}",
            self.red, self.green, self.blue
        ))
    }
}

impl Debug for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "RGB(#{:x}{:x}{:x})",
            self.red, self.green, self.blue
        ))
    }
}

impl From<RGB> for u32 {
    fn from(value: RGB) -> Self {
        (value.red as u32) << 16 | (value.green as u32) << 8 | value.blue as u32
    }
}
