#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::{double_line_ending, multi_line_parser, Parsable};
use fxhash::FxHashMap;
use itertools::Either::{Left, Right};
use itertools::{Either, Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::{complete, is_alphanumeric};
use nom::combinator::map;
use nom::error::{Error, ErrorKind};
use nom::sequence::{separated_pair, tuple};
use nom::{IResult, InputTake, Parser};
use std::fmt::{Display, Formatter};
use std::mem::swap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name([u8; 3]);

fn parse_name(input: &[u8]) -> IResult<&[u8], Name> {
    if input.len() < 3 {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Eof)));
    }

    let (rest, name) = input.take_split(3);
    if !name.iter().all(|&b| is_alphanumeric(b)) {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::AlphaNumeric)));
    }

    Ok((rest, Name([name[0], name[1], name[2]])))
}

struct StartingValue(Name, bool);

impl Parsable for StartingValue {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(parse_name, tag(b": "), complete::u8),
            |(name, value)| StartingValue(name, value != 0),
        )
    }
}

impl Name {
    fn from(prefix: u8, id: u16) -> Name {
        Name([prefix, (id / 10) as u8 + b'0', (id % 10) as u8 + b'0'])
    }

    fn wired_from(&self, left: Name, operation: Operation, right: Name) -> Expression {
        Expression { left, operation, right, target: *self }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Operation {
    And,
    Xor,
    Or,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "AND"),
            Operation::Xor => write!(f, "XOR"),
            Operation::Or => write!(f, "OR"),
        }
    }
}

#[derive(Clone)]
struct Expression {
    left: Name,
    operation: Operation,
    right: Name,
    target: Name,
}

impl Expression {
    fn get_other_input(&self, name: &Name) -> Option<&Name> {
        if &self.left == name {
            Some(&self.right)
        } else if &self.right == name {
            Some(&self.left)
        } else {
            None
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation
            && ((self.left == other.left && self.right == other.right)
                || (self.left == other.right && self.right == other.left))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.left, self.operation, self.right, self.target
        )
    }
}

impl Parsable for Expression {
    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(
                tuple((
                    parse_name,
                    alt((
                        map(tag(b" AND "), |_| Operation::And),
                        map(tag(b" XOR "), |_| Operation::Xor),
                        map(tag(b" OR "), |_| Operation::Or),
                    )),
                    parse_name,
                )),
                tag(b" -> "),
                parse_name,
            ),
            |((left, operation, right), target)| Expression { left, operation, right, target },
        )
    }
}

struct Day {
    starting_values: FxHashMap<Name, bool>,
    expressions: FxHashMap<Name, Expression>,
}

impl Day {
    fn find_target_of_expression(&self, expected: &Expression) -> Option<Name> {
        let result = self.expressions.iter().find(|(_, expr)| expr == &expected).map(|(n, _)| *n);
        if result.is_none() {
            println!("Could not find expression: {}", expected);
        }
        result
    }

    fn find_expression(&self, target: Name) -> Option<&Expression> {
        let result = self.expressions.get(&target);
        if result.is_none() {
            println!("Could not find expression for target: {}", target);
        }
        result
    }

    fn evaluate(&self, name: &Name) -> Option<bool> {
        if let Some(&value) = self.starting_values.get(name) {
            return Some(value);
        }

        let expression = self.expressions.get(name)?;
        let left = self.evaluate(&expression.left)?;
        let right = self.evaluate(&expression.right)?;
        match expression.operation {
            Operation::And => Some(left & right),
            Operation::Xor => Some(left ^ right),
            Operation::Or => Some(left | right),
        }
    }

    fn detect_half_adder(&self, id: u16) -> Option<Name> {
        let target = Name::from(b'z', id);
        let from_x = Name::from(b'x', id);
        let from_y = Name::from(b'y', id);

        if let Some(expr) = self.expressions.get(&target) {
            if expr == &target.wired_from(from_x, Operation::Xor, from_y) {
                self.find_target_of_expression(&target.wired_from(from_x, Operation::And, from_y))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn detect_full_adder(&self, id: u16, carry_in: Name) -> Either<Name, (Name, Name)> {
        let target = Name::from(b'z', id);
        let from_x = Name::from(b'x', id);
        let from_y = Name::from(b'y', id);

        let carry_in_expr = self.find_expression(target).unwrap();
        let source_target = carry_in_expr.get_other_input(&carry_in);
        if source_target.is_none() || carry_in_expr.operation != Operation::Xor {
            let other_target = self
                .expressions
                .values()
                .find(|expr| {
                    expr.operation == Operation::Xor
                        && (expr.left == carry_in || expr.right == carry_in)
                })
                .unwrap()
                .target;

            return Right((target, other_target));
        }
        let source_target = *source_target.unwrap();

        let from_expr = self.find_expression(source_target).unwrap();
        let expected_expr = target.wired_from(from_x, Operation::Xor, from_y);
        if from_expr != &expected_expr {
            let other_target = self
                .expressions
                .values()
                .find(|expr| {
                    expr.operation == Operation::Xor
                        && (expr.left == from_x || expr.right == from_x)
                        && (expr.left == from_y || expr.right == from_y)
                })
                .unwrap()
                .target;

            return Right((source_target, other_target));
        }

        let carry_out_left = self
            .find_target_of_expression(&target.wired_from(source_target, Operation::And, carry_in))
            .unwrap();

        let carry_out_right = self
            .find_target_of_expression(&target.wired_from(from_x, Operation::And, from_y))
            .unwrap();

        Left(
            self.find_target_of_expression(&target.wired_from(
                carry_out_left,
                Operation::Or,
                carry_out_right,
            ))
            .unwrap(),
        )
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_pair(
                multi_line_parser::<StartingValue>(),
                double_line_ending,
                multi_line_parser::<Expression>(),
            ),
            |(starting_values, expressions)| Day {
                starting_values: starting_values
                    .into_iter()
                    .map(|starting_value| (starting_value.0, starting_value.1))
                    .collect(),
                expressions: expressions.into_iter().map(|expr| (expr.target, expr)).collect(),
            },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut result = 0;
        for ix in 0..64 {
            let name = Name(format!("z{:02}", ix).as_bytes().try_into().unwrap());
            if !self.expressions.contains_key(&name) {
                break;
            }

            if let Some(value) = self.evaluate(&name) {
                if value {
                    result |= 1 << ix;
                }
            }
        }

        result
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut fixed_day = Day {
            starting_values: self.starting_values.clone(),
            expressions: self.expressions.clone(),
        };
        let mut swapped_names = Vec::new();

        println!(" ├── Part 2, detecting the adders");
        let mut carry = fixed_day.detect_half_adder(0).unwrap();
        let mut index = 1;
        while index < 45 {
            match fixed_day.detect_full_adder(index, carry) {
                Left(next_carry) => {
                    carry = next_carry;
                    index += 1;
                }
                Right((source_target, other_target)) => {
                    println!("  ├── Swapping {} and {}", source_target, other_target);
                    swapped_names.push(source_target);
                    swapped_names.push(other_target);

                    let mut source_expr = fixed_day.expressions.remove(&source_target).unwrap();
                    let mut other_expr = fixed_day.expressions.remove(&other_target).unwrap();

                    swap(&mut source_expr.target, &mut other_expr.target);
                    fixed_day.expressions.insert(other_target, source_expr);
                    fixed_day.expressions.insert(source_target, other_expr);
                }
            }
        }

        swapped_names.sort();
        println!(
            " ├── Part 2 result: {}",
            swapped_names.iter().map(|n| n.to_string()).join(",")
        );

        swapped_names.len() as u64
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 24, example1 => 4 );
    day_test!( 24, example2 => 2024 );
    day_test!( 24 => 48508229772400, 8 );
}
