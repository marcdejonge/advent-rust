use fxhash::FxHashMap;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map};
use nom::error::{Error, ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{Compare, Err, Finish, IResult, InputIter, InputLength, InputTake, Parser, Slice};
use smallvec::SmallVec;
use std::hash::Hash;
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
                Err(Err::Error(_)) if !result.is_empty() => break,
                Err(e) => return Err(e),
                Ok((next_input, output)) => {
                    // infinite loop check: the parser must always consume
                    if next_input.input_len() == input_len_before {
                        return Err(Err::Error(Error::from_error_kind(input, ErrorKind::Many1)));
                    }
                    if result.len() == MAX {
                        return Err(Err::Error(Error::from_error_kind(input, ErrorKind::ManyMN)));
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

pub fn separated_array<'a, const D: usize, T, S>(
    mut separator: impl Parser<&'a [u8], S, Error<&'a [u8]>>,
) -> impl Parser<&'a [u8], [T; D], Error<&'a [u8]>>
where
    T: Parsable + Default + Copy,
{
    move |input: &'a [u8]| {
        let mut input = input;
        let mut result = [T::default(); D];
        let mut first = true;
        for value in result.iter_mut() {
            if !first {
                let (rest, _) = separator.parse(input)?;
                input = rest;
            } else {
                first = false;
            }

            let (rest, next_value) = T::parser().parse(input)?;
            *value = next_value;
            input = rest;
        }
        Ok((input, result))
    }
}

macro_rules! number_parsable {
    ($($t:ty => $i:ident),*) => {
        $(
            impl Parsable for $t {
                fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
                    nom::character::complete::$i
                }
            }
        )*
    };
}

number_parsable!(u8 => u8, u16 => u16, u32 => u32, u64 => u64, u128 => u128);
number_parsable!(i8 => i8, i16 => i16, i32 => i32, i64 => i64, i128 => i128);

pub fn parsable_pair<'a, O1, O2, S>(
    separator: impl Parser<&'a [u8], S, Error<&'a [u8]>>,
) -> impl Parser<&'a [u8], (O1, O2), Error<&'a [u8]>>
where
    O1: Parsable,
    O2: Parsable,
{
    separated_pair(O1::parser(), separator, O2::parser())
}

pub fn parsable_triple<'a, O1, O2, O3, S>(
    separator: impl Parser<&'a [u8], S, Error<&'a [u8]>> + Clone,
) -> impl Parser<&'a [u8], (O1, O2, O3), Error<&'a [u8]>>
where
    O1: Parsable,
    O2: Parsable,
    O3: Parsable,
{
    map(
        tuple((
            O1::parser(),
            separator.clone(),
            O2::parser(),
            separator,
            O3::parser(),
        )),
        |(o1, _, o2, _, o3)| (o1, o2, o3),
    )
}

pub fn double_line_ending<I>(input: I) -> IResult<I, (I, I)>
where
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength + Clone,
    I: Compare<&'static str>,
{
    tuple((line_ending, line_ending))(input)
}

pub fn separated_lines1<'a, T>() -> impl Parser<&'a [u8], Vec<T>, Error<&'a [u8]>>
where
    T: Parsable,
{
    separated_list1(line_ending, T::parser())
}

pub fn separated_double_lines1<'a, T>() -> impl Parser<&'a [u8], Vec<T>, Error<&'a [u8]>>
where
    T: Parsable,
{
    separated_list1(double_line_ending, T::parser())
}

pub fn map_parsable<'a, T, U, F>(function: F) -> impl Parser<&'a [u8], U, Error<&'a [u8]>>
where
    T: Parsable,
    F: FnMut(T) -> U,
{
    map(T::parser(), function)
}

pub fn in_parens<'a, O>(
    parser: impl Parser<&'a [u8], O, Error<&'a [u8]>>,
) -> impl Parser<&'a [u8], O, Error<&'a [u8]>> {
    delimited(tag(b"("), parser, tag(b")"))
}

pub fn in_brackets<'a, O>(
    parser: impl Parser<&'a [u8], O, Error<&'a [u8]>>,
) -> impl Parser<&'a [u8], O, Error<&'a [u8]>> {
    delimited(tag(b"["), parser, tag(b"]"))
}

pub fn in_braces<'a, O>(
    parser: impl Parser<&'a [u8], O, Error<&'a [u8]>>,
) -> impl Parser<&'a [u8], O, Error<&'a [u8]>> {
    delimited(tag(b"{"), parser, tag(b"}"))
}

pub fn separated_map1<'a, S, K, V>(
    separator: impl Parser<&'a [u8], S, Error<&'a [u8]>>,
    parser: impl Parser<&'a [u8], (K, V), Error<&'a [u8]>>,
) -> impl Parser<&'a [u8], FxHashMap<K, V>, Error<&'a [u8]>>
where
    K: Eq + Hash,
{
    map(separated_list1(separator, parser), |list| {
        list.into_iter().collect::<FxHashMap<K, V>>()
    })
}

pub fn map_to_vec<'a, 'b, T: Clone + 'b, ParserIn>(
    parser: ParserIn,
) -> impl Parser<&'a [u8], Vec<T>, Error<&'a [u8]>> + use<'a, 'b, T, ParserIn>
where
    ParserIn: Parser<&'a [u8], &'b [T], Error<&'a [u8]>>,
{
    map(parser, |slice: &[T]| slice.to_vec())
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
