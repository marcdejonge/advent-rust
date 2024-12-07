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
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[inline]
pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
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
