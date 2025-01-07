use nom::bytes::complete::{tag, take_while_m_n};
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::{AsChar, Compare, IResult, InputIter, InputLength, InputTake, Parser, Slice};
use nom_parse_trait::ParseFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeFrom;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl<I, E> ParseFrom<I, E> for RGB
where
    E: ParseError<I>,
    I: Clone + InputTake + InputLength + InputIter,
    <I as InputIter>::Item: AsChar + Copy,
    I: Slice<RangeFrom<usize>>,
    I: Compare<&'static str>,
{
    fn parse(input: I) -> IResult<I, Self, E> {
        fn parse_hex([a, b]: [impl AsChar; 2]) -> u8 {
            let a = a.as_char();
            let b = b.as_char();
            (a.to_digit(16).unwrap() * 16 + b.to_digit(16).unwrap()) as u8
        }

        let (rest, parsed) =
            preceded(tag("#"), take_while_m_n(6, 6, AsChar::is_hex_digit)).parse(input)?;

        let mut iter = parsed.iter_elements().array_chunks();
        let red = iter.next().map(parse_hex).unwrap();
        let green = iter.next().map(parse_hex).unwrap();
        let blue = iter.next().map(parse_hex).unwrap();
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
