use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::{IResult, InputLength, InputTake, Parser};

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
