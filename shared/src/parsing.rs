use fxhash::{FxHashMap, FxHashSet};
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map};
use nom::error::{ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{
    AsBytes, Compare, Err, Finish, IResult, InputIter, InputLength, InputTake, Parser, Slice,
};
use nom_parse_trait::ParseFrom;
use smallvec::SmallVec;
use std::hash::Hash;
use std::ops::{Range, RangeFrom, RangeTo};

pub fn handle_parser_error<T>(input: &[u8]) -> T
where
    T: for<'a> ParseFrom<&'a [u8]>,
{
    match all_consuming(T::parse).parse(input).finish() {
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
pub fn find_many_skipping_unknown<I, E, T>() -> impl Parser<I, Vec<T>, E>
where
    T: ParseFrom<I, E>,
    I: Clone + InputLength + Slice<RangeFrom<usize>>,
{
    move |mut input: I| {
        let mut res = Vec::new();
        while input.input_len() > 0 {
            let value = T::parse(input.clone());
            match value {
                Ok((left, o)) => {
                    res.push(o);
                    input = left;
                }
                Err(_) => input = input.slice(1..),
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

pub fn separated_array<I, E, const D: usize, T, S>(
    mut separator: impl Parser<I, S, E>,
) -> impl Parser<I, [T; D], E>
where
    T: ParseFrom<I, E> + Default + Copy,
{
    move |input: I| {
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

            let (rest, next_value) = T::parse(input)?;
            *value = next_value;
            input = rest;
        }
        Ok((input, result))
    }
}

pub fn parsable_pair<I, E, O1, O2, S>(
    separator: impl Parser<I, S, E>,
) -> impl Parser<I, (O1, O2), E>
where
    E: ParseError<I>,
    O1: ParseFrom<I, E>,
    O2: ParseFrom<I, E>,
{
    separated_pair(O1::parse, separator, O2::parse)
}

pub fn parsable_triple<I, E, O1, O2, O3, S>(
    separator: impl Parser<I, S, E> + Clone,
) -> impl Parser<I, (O1, O2, O3), E>
where
    I: Clone,
    E: ParseError<I>,
    O1: ParseFrom<I, E>,
    O2: ParseFrom<I, E>,
    O3: ParseFrom<I, E>,
{
    map(
        tuple((
            O1::parse,
            separator.clone(),
            O2::parse,
            separator,
            O3::parse,
        )),
        |(o1, _, o2, _, o3)| (o1, o2, o3),
    )
}

pub fn double_line_ending<I, E>(input: I) -> IResult<I, (I, I), E>
where
    E: ParseError<I>,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength + Clone,
    I: Compare<&'static str>,
{
    tuple((line_ending, line_ending))(input)
}

pub fn separated_lines1<I, E, T>() -> impl Parser<I, Vec<T>, E>
where
    E: ParseError<I>,
    T: ParseFrom<I, E>,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength + Clone,
    I: Compare<&'static str>,
{
    separated_list1(line_ending, T::parse)
}

pub fn separated_double_lines1<I, E, T>() -> impl Parser<I, Vec<T>, E>
where
    T: ParseFrom<I, E>,
    E: ParseError<I>,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength + Clone,
    I: Compare<&'static str>,
{
    separated_list1(double_line_ending, T::parse)
}

pub fn map_parsable<I, E, T, U, F>(function: F) -> impl Parser<I, U, E>
where
    T: ParseFrom<I, E>,
    F: FnMut(T) -> U,
{
    map(T::parse, function)
}

pub fn single<I, E>(b: u8) -> impl Parser<I, u8, E>
where
    E: ParseError<I>,
    I: AsBytes + InputLength + InputTake + Clone,
{
    single_match(move |c| c == b)
}

pub fn single_space<I, E>() -> impl Parser<I, u8, E>
where
    E: ParseError<I>,
    I: AsBytes + InputLength + InputTake + Clone,
{
    single(b' ')
}

pub fn single_digit<I, E>() -> impl Parser<I, u8, E>
where
    E: ParseError<I>,
    I: AsBytes + InputLength + InputTake + Clone,
{
    single_match(|b| b.is_ascii_digit())
}

pub fn single_match<I, E>(mut matcher: impl FnMut(u8) -> bool) -> impl Parser<I, u8, E>
where
    E: ParseError<I>,
    I: AsBytes + InputLength + InputTake + Clone,
{
    move |input: I| {
        if input.input_len() == 0 {
            return Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof)));
        }

        let (rest, taken) = input.clone().take_split(1);
        let bytes = taken.as_bytes();
        let b = taken.as_bytes()[0];
        if bytes.len() == 1 && matcher(b) {
            Ok((rest, b))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Char)))
        }
    }
}

pub fn in_parens<I, E, O>(parser: impl Parser<I, O, E>) -> impl Parser<I, O, E>
where
    I: Clone + InputLength + AsBytes + InputTake,
    E: ParseError<I>,
{
    delimited(single(b'('), parser, single(b')'))
}

pub fn in_brackets<I, E, O>(parser: impl Parser<I, O, E>) -> impl Parser<I, O, E>
where
    I: Clone + InputLength + AsBytes + InputTake,
    E: ParseError<I>,
{
    delimited(single(b'['), parser, single(b']'))
}

pub fn in_braces<I, E, O>(parser: impl Parser<I, O, E>) -> impl Parser<I, O, E>
where
    I: Clone + InputLength + AsBytes + InputTake,
    E: ParseError<I>,
{
    delimited(single(b'{'), parser, single(b'}'))
}

pub fn separated_map1<I, E, S, K, V>(
    separator: impl Parser<I, S, E>,
    parser: impl Parser<I, (K, V), E>,
) -> impl Parser<I, FxHashMap<K, V>, E>
where
    E: ParseError<I>,
    I: Clone + InputLength,
    K: Eq + Hash,
{
    map(separated_list1(separator, parser), |list| {
        list.into_iter().collect::<FxHashMap<K, V>>()
    })
}

pub fn separated_set1<I, E, T, S>(
    separator: impl Parser<I, S, E>,
    parser: impl Parser<I, T, E>,
) -> impl Parser<I, FxHashSet<T>, E>
where
    T: Eq + Hash,
    E: ParseError<I>,
    I: Clone + InputLength,
{
    map(separated_list1(separator, parser), |list| {
        list.into_iter().collect::<FxHashSet<T>>()
    })
}
