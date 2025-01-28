use fxhash::{FxHashMap, FxHashSet};
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, map};
use nom::error::{ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::{AsBytes, Compare, Err, Finish, IResult, Input, Parser};
use nom_parse_trait::ParseFrom;
use smallvec::SmallVec;
use std::hash::Hash;

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
pub fn find_many_skipping_unknown<I: Input, E: ParseError<I>, T: ParseFrom<I, E>>(
) -> impl Parser<I, Output = Vec<T>, Error = E> {
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

pub fn many_1_n<const MAX: usize, Input, Output, Error, ParserFunction>(
    mut parser: ParserFunction,
) -> impl FnMut(Input) -> IResult<Input, SmallVec<[Output; MAX]>, Error>
where
    Input: nom::Input,
    ParserFunction: Parser<Input, Output = Output, Error = Error>,
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

pub fn separated_array<I: Input, E: ParseError<I>, const D: usize, T, S>(
    mut separator: impl Parser<I, Output = S, Error = E>,
) -> impl Parser<I, Output = [T; D], Error = E>
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
