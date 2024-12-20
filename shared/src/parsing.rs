use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::error::{Error, ParseError};
use nom::{Finish, IResult, InputLength, InputTake, Parser};
use smallvec::SmallVec;

#[inline]
pub fn find_many<Input, Output, Error, ParseFunction>(
    mut f: ParseFunction,
    mut input: Input,
) -> Vec<Output>
where
    Input: Clone + InputLength + InputTake,
    ParseFunction: Parser<Input, Output, Error>,
{
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
    res
}

#[inline]
pub fn digits<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse::<T>())(input)
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

pub fn full<Input: InputLength, Output>(
    mut parser: impl Parser<Input, Output, Error<Input>>,
) -> impl FnMut(Input) -> Result<Output, Error<Input>> {
    move |input| {
        parser.parse(input).finish().and_then(|(rest, output)| {
            if rest.input_len() == 0 {
                Ok(output)
            } else {
                Err(Error::from_error_kind(
                    rest,
                    nom::error::ErrorKind::Complete,
                ))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::character::complete::char;
    use nom::sequence::pair;

    #[test]
    fn test_find0() {
        let input = "a1b2c3";
        let res = find_many::<_, _, (), _>(pair(char('a'), char('1')), input);
        assert_eq!(res, vec![('a', '1')]);
    }

    #[test]
    fn test_find0_many() {
        let input = "a1b2a1b2a1";
        let res = find_many::<_, _, (), _>(pair(char('a'), char('1')), input);
        assert_eq!(res, vec![('a', '1'); 3]);
    }
}
