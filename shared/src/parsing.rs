use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map};
use nom::error::{Error, ParseError};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{Compare, Finish, IResult, InputIter, InputLength, InputTake, Parser, Slice};
use smallvec::SmallVec;
use std::ops::{Range, RangeFrom, RangeTo};

pub fn handle_parser_error<'a, T>(
    input: &'a [u8],
    parser: impl Parser<&'a [u8], T, Error<&'a [u8]>>,
) -> T {
    match all_consuming(parser).parse(input).finish() {
        Ok((_, day)) => day,
        Err(e) => {
            panic!(
                "Error parsing input, code: {:?}. Rest input:\n{}",
                e.code,
                String::from_utf8_lossy(if e.input.len() > 100 {
                    &e.input[0..100]
                } else {
                    e.input
                })
            );
        }
    }
}

#[inline]
pub fn find_many_skipping_unknown<Input, Output, Error, ParseFunction>(
    mut f: ParseFunction,
) -> impl Parser<Input, Vec<Output>, Error>
where
    Input: Clone + InputLength + InputTake,
    ParseFunction: Parser<Input, Output, Error>,
{
    move |mut input: Input| {
        let mut res = Vec::new();
        while input.input_len() > 0 {
            let value = f.parse(input.clone());
            match value {
                Ok((left, o)) => {
                    res.push(o);
                    input = left;
                }
                Err(_) => input = input.take_split(1).0,
            }
        }
        Ok((input, res))
    }
}

pub fn many_1_n<const MAX: usize, Input, Output, Error, ParserFunction>(
    mut parser: ParserFunction,
) -> impl FnMut(Input) -> IResult<Input, SmallVec<[Output; MAX]>, Error>
where
    Input: Clone + InputLength,
    ParserFunction: Parser<Input, Output, Error>,
    Error: ParseError<Input>,
    [Output; MAX]: smallvec::Array<Item = Output>,
{
    move |mut input: Input| {
        let mut result = SmallVec::new();
        while result.len() < MAX {
            let input_len_before = input.input_len();
            match parser.parse(input.clone()) {
                Err(nom::Err::Error(_)) if !result.is_empty() => break,
                Err(e) => return Err(e),
                Ok((next_input, output)) => {
                    // infinite loop check: the parser must always consume
                    if next_input.input_len() == input_len_before {
                        return Err(nom::Err::Error(Error::from_error_kind(
                            input,
                            nom::error::ErrorKind::Many1,
                        )));
                    }
                    if result.len() == MAX {
                        return Err(nom::Err::Error(Error::from_error_kind(
                            input,
                            nom::error::ErrorKind::ManyMN,
                        )));
                    }

                    input = next_input;
                    result.push(output);
                }
            }
        }

        Ok((input, result))
    }
}

pub trait Parsable
where
    Self: Sized,
{
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>>;
}

pub fn double_line_ending<I>(input: I) -> IResult<I, (I, I)>
where
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength + Clone,
    I: Compare<&'static str>,
{
    tuple((line_ending, line_ending))(input)
}

pub fn multi_line_parser<'a, T>() -> impl Parser<&'a [u8], Vec<T>, Error<&'a [u8]>>
where
    T: Parsable,
{
    separated_list1(line_ending, T::parser())
}

pub fn double_line_parser<'a, T>() -> impl Parser<&'a [u8], Vec<T>, Error<&'a [u8]>>
where
    T: Parsable,
{
    separated_list1(tuple((line_ending, line_ending)), T::parser())
}

pub fn map_parser<'a, T, U, F>(function: F) -> impl Parser<&'a [u8], U, Error<&'a [u8]>>
where
    T: Parsable,
    F: FnMut(T) -> U,
{
    map(T::parser(), function)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::character::complete::char;
    use nom::sequence::pair;

    #[test]
    fn test_find0() {
        let input = "a1b2c3";
        let res =
            find_many_skipping_unknown::<_, _, (), _>(pair(char('a'), char('1'))).parse(input);
        assert_eq!(res, Ok(("", vec![('a', '1')])));
    }

    #[test]
    fn test_find0_many() {
        let input = "a1b2a1b2a1";
        let res =
            find_many_skipping_unknown::<_, _, (), _>(pair(char('a'), char('1'))).parse(input);
        assert_eq!(res, Ok(("", vec![('a', '1'); 3])));
    }
}
