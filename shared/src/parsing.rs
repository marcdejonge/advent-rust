use crate::builder::with;
use fxhash::{FxHashMap, FxHashSet};
use nom::character::complete::{line_ending, newline};
use nom::combinator::{all_consuming, map};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{AsBytes, AsChar, Compare, Err, Finish, IResult, Input, Parser};
use nom_parse_trait::ParseFrom;
use smallvec::SmallVec;
use std::hash::Hash;
use std::ops::RangeInclusive;

pub fn handle_parser_error<T>(input: &[u8]) -> T
where
    T: for<'a> ParseFrom<&'a [u8]>,
{
    match all_consuming(terminated(T::parse, many0(newline)))
        .parse_complete(input)
        .finish()
    {
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
pub fn find_many_skipping_unknown<I: Input, E: ParseError<I>, T: ParseFrom<I, E>>()
-> impl Parser<I, Output = Vec<T>, Error = E> {
    move |mut input: I| {
        let mut res = Vec::new();
        while input.input_len() > 0 {
            let value = T::parse(input.clone());
            match value {
                Ok((left, o)) => {
                    res.push(o);
                    input = left;
                }
                Err(_) => input = input.take_from(1),
            }
        }
        Ok((input, res))
    }
}

pub fn many_1_n<const MAX: usize, I, Output, E, ParserFunction>(
    mut parser: ParserFunction,
) -> impl Parser<I, Output = SmallVec<[Output; MAX]>, Error = E>
where
    I: Input,
    ParserFunction: Parser<I, Output = Output, Error = E>,
    E: ParseError<I>,
    [Output; MAX]: smallvec::Array<Item = Output>,
{
    move |mut input: I| {
        let mut result = SmallVec::new();
        while result.len() < MAX {
            let input_len_before = input.input_len();
            match parser.parse(input.clone()) {
                Err(Err::Error(_)) if !result.is_empty() => break,
                Err(e) => return Err(e),
                Ok((next_input, output)) => {
                    // infinite loop check: the parser must always consume
                    if next_input.input_len() == input_len_before {
                        return Err(Err::Error(E::from_error_kind(input, ErrorKind::Many1)));
                    }
                    if result.len() == MAX {
                        return Err(Err::Error(E::from_error_kind(input, ErrorKind::ManyMN)));
                    }

                    input = next_input;
                    result.push(output);
                }
            }
        }

        Ok((input, result))
    }
}

pub fn separated_array<I: Input, E: ParseError<I>, const D: usize, T, S>(
    separator: impl Parser<I, Output = S, Error = E>,
) -> impl Parser<I, Output = [T; D], Error = E>
where
    T: ParseFrom<I, E> + Default + Copy,
{
    separated_array_with(separator, T::parse)
}

pub fn separated_array_with<I: Input, E: ParseError<I>, const D: usize, T, S>(
    mut separator: impl Parser<I, Output = S, Error = E>,
    mut parser: impl Parser<I, Output = T, Error = E>,
) -> impl Parser<I, Output = [T; D], Error = E>
where
    T: Default + Copy,
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

            let (rest, next_value) = parser.parse(input)?;
            *value = next_value;
            input = rest;
        }
        Ok((input, result))
    }
}

pub fn parsable_pair<I, E, O1, O2, S>(
    separator: impl Parser<I, Output = S, Error = E>,
) -> impl Parser<I, Output = (O1, O2), Error = E>
where
    E: ParseError<I>,
    O1: ParseFrom<I, E>,
    O2: ParseFrom<I, E>,
{
    separated_pair(O1::parse, separator, O2::parse)
}

pub fn parsable_triple<I, E, O1, O2, O3, S>(
    separator: impl Parser<I, Output = S, Error = E> + Clone,
) -> impl Parser<I, Output = (O1, O2, O3), Error = E>
where
    I: Clone,
    E: ParseError<I>,
    O1: ParseFrom<I, E>,
    O2: ParseFrom<I, E>,
    O3: ParseFrom<I, E>,
{
    map(
        (
            O1::parse,
            separator.clone(),
            O2::parse,
            separator,
            O3::parse,
        ),
        |(o1, _, o2, _, o3)| (o1, o2, o3),
    )
}

pub fn double_line_ending<I, E>(input: I) -> IResult<I, (I, I), E>
where
    E: ParseError<I>,
    I: Input + Compare<&'static str>,
{
    (line_ending, line_ending).parse(input)
}

pub fn separated_lines1<I, E, T>() -> impl Parser<I, Output = Vec<T>, Error = E>
where
    E: ParseError<I>,
    T: ParseFrom<I, E>,
    I: Input + Compare<&'static str>,
{
    separated_list1(line_ending, T::parse)
}

pub fn separated_double_lines1<I, E, T>() -> impl Parser<I, Output = Vec<T>, Error = E>
where
    T: ParseFrom<I, E>,
    E: ParseError<I>,
    I: Input + Compare<&'static str>,
{
    separated_list1(double_line_ending, T::parse)
}

pub fn map_parsable<I: Input, E: ParseError<I>, T: ParseFrom<I, E>, U, F: FnMut(T) -> U>(
    function: F,
) -> impl Parser<I, Output = U, Error = E> {
    map(T::parse, function)
}

pub fn single<I: AsBytes + Input, E: ParseError<I>>(
    b: u8,
) -> impl Parser<I, Output = u8, Error = E> {
    single_match(move |c| c == b)
}

pub fn single_space<I: AsBytes + Input, E: ParseError<I>>() -> impl Parser<I, Output = u8, Error = E>
{
    single(b' ')
}

pub fn single_digit<I: AsBytes + Input, E: ParseError<I>>() -> impl Parser<I, Output = u8, Error = E>
{
    single_match(|b| b.is_ascii_digit())
}

pub fn single_ascii<I: AsBytes + Input, E: ParseError<I>>() -> impl Parser<I, Output = u8, Error = E>
{
    single_match(|b| b.is_ascii_alphabetic())
}

pub fn single_match<I: AsBytes + Input, E: ParseError<I>>(
    mut matcher: impl FnMut(u8) -> bool,
) -> impl Parser<I, Output = u8, Error = E> {
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

pub fn in_parens<I: AsBytes + Input, E: ParseError<I>, O>(
    parser: impl Parser<I, Output = O, Error = E>,
) -> impl Parser<I, Output = O, Error = E> {
    delimited(single(b'('), parser, single(b')'))
}

pub fn in_brackets<I: AsBytes + Input, E: ParseError<I>, O>(
    parser: impl Parser<I, Output = O, Error = E>,
) -> impl Parser<I, Output = O, Error = E> {
    delimited(single(b'['), parser, single(b']'))
}

pub fn in_braces<I: AsBytes + Input, E: ParseError<I>, O>(
    parser: impl Parser<I, Output = O, Error = E>,
) -> impl Parser<I, Output = O, Error = E> {
    delimited(single(b'{'), parser, single(b'}'))
}

pub fn separated_map1<I: Input, E: ParseError<I>, S, K: Eq + Hash, V>(
    separator: impl Parser<I, Output = S, Error = E>,
    parser: impl Parser<I, Output = (K, V), Error = E>,
) -> impl Parser<I, Output = FxHashMap<K, V>, Error = E> {
    map(separated_list1(separator, parser), |list| {
        list.into_iter().collect::<FxHashMap<K, V>>()
    })
}

pub fn separated_set1<I: Input, E: ParseError<I>, T: Eq + Hash, S>(
    separator: impl Parser<I, Output = S, Error = E>,
    parser: impl Parser<I, Output = T, Error = E>,
) -> impl Parser<I, Output = FxHashSet<T>, Error = E> {
    map(separated_list1(separator, parser), |list| {
        list.into_iter().collect::<FxHashSet<T>>()
    })
}

pub fn peek_char_mapped<I: Input, E: ParseError<I>, T>(
    transform: impl Fn(&char) -> T,
) -> impl Parser<I, Output = T, Error = E>
where
    <I as Input>::Item: AsChar,
{
    move |input: I| {
        let first = input
            .clone()
            .iter_elements()
            .next()
            .ok_or_else(|| Err::Error(E::from_error_kind(input.clone(), ErrorKind::Eof)))?;
        Ok((input, transform(&first.as_char())))
    }
}

pub fn parser_to_string<I: Input, E: ParseError<I>>(
    parser: impl Parser<I, Output = I, Error = E>,
) -> impl Parser<I, Output = String, Error = E>
where
    <I as Input>::Item: AsChar,
{
    map(parser, |chars: I| {
        with(String::new(), |it| {
            for char in chars.iter_elements() {
                it.push(char.as_char());
            }
        })
    })
}

pub fn char_alpha<I: Input, E: ParseError<I>>() -> impl Parser<I, Output = char, Error = E>
where
    <I as Input>::Item: AsChar,
{
    single_char_match(|c| c.is_alpha())
}

pub fn single_char_match<I: Input, E: ParseError<I>>(
    matcher: impl Fn(char) -> bool,
) -> impl Parser<I, Output = char, Error = E>
where
    <I as Input>::Item: AsChar,
{
    move |input: I| match input.iter_elements().next() {
        None => Err(Err::Error(E::from_error_kind(
            input.clone(),
            ErrorKind::Eof,
        ))),
        Some(item) => {
            let char = item.as_char();
            if matcher(char) {
                Ok((input.take_from(1), char))
            } else {
                Err(Err::Error(E::from_error_kind(input, ErrorKind::Char)))
            }
        }
    }
}

pub fn hex8<I: Input, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
    <I as Input>::Item: AsChar,
{
    if input.input_len() < 2 {
        return Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof)));
    }

    let (rest, matches) = input.take_split(2);
    let mut iter = matches.iter_elements();
    let first = hex_decode(
        iter.next()
            .ok_or_else(|| Err::Error(E::from_error_kind(input.clone(), ErrorKind::Eof)))?
            .as_char(),
    )
    .ok_or_else(|| Err::Error(E::from_error_kind(input.clone(), ErrorKind::Char)))?;
    let second = hex_decode(
        iter.next()
            .ok_or_else(|| Err::Error(E::from_error_kind(input.clone(), ErrorKind::Eof)))?
            .as_char(),
    )
    .ok_or_else(|| Err::Error(E::from_error_kind(input.clone(), ErrorKind::Char)))?;

    Ok((rest, first << 4 | second))
}

fn hex_decode(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        _ => None,
    }
}

pub fn range_inclusive<I, E: ParseError<I>, T, F, G>(
    value: F,
    separator: G,
) -> impl Parser<I, Output = RangeInclusive<T>, Error = E>
where
    F: Parser<I, Output = T, Error = E> + Clone,
    G: Parser<I, Error = E>,
    I: Input,
    <I as Input>::Item: AsChar,
    for<'a> I: Compare<&'a [u8]>,
{
    map(separated_pair(value.clone(), separator, value), |(l, r)| {
        l..=r
    })
}
