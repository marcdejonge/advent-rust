use nom::bytes::complete::{tag, take_while_m_n};
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::{AsChar, Compare, IResult, Input, Parser};
use nom_parse_trait::ParseFrom;
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl<I, E> ParseFrom<I, E> for RGB
where
    E: ParseError<I>,
    I: Input,
    <I as Input>::Item: AsChar + Copy,
    I: Compare<&'static str>,
{
    fn parse(input: I) -> IResult<I, Self, E> {
        fn parse_hex(a: impl AsChar, b: impl AsChar) -> u8 {
            (a.as_char().to_digit(16).unwrap() * 16 + b.as_char().to_digit(16).unwrap()) as u8
        }

        let (rest, parsed) =
            preceded(tag("#"), take_while_m_n(6, 6, AsChar::is_hex_digit)).parse(input)?;

        let mut chars = parsed.iter_elements();
        let red = parse_hex(chars.next().unwrap(), chars.next().unwrap());
        let green = parse_hex(chars.next().unwrap(), chars.next().unwrap());
        let blue = parse_hex(chars.next().unwrap(), chars.next().unwrap());
        Ok((rest, RGB { red, green, blue }))
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
