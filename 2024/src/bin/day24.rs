#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::full;
use fxhash::FxHashMap;
use itertools::Either::{Left, Right};
use itertools::{Either, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1};
use nom::sequence::tuple;
use std::fmt::{Display, Formatter};
use std::mem::swap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name([u8; 3]);

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
        write!(
            f,
            "{}{}{}",
            self.0[0] as char, self.0[1] as char, self.0[2] as char
        )
    }
}

fn parse_name(s: &str) -> (Name, bool) {
    let (name, _, nr) = full(tuple((alphanumeric1, tag(": "), digit1)))(s).unwrap();
    (Name(name.as_bytes().try_into().unwrap()), nr != "0")
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Operation {
    AND,
    XOR,
    OR,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::AND => write!(f, "AND"),
            Operation::XOR => write!(f, "XOR"),
            Operation::OR => write!(f, "OR"),
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

fn parse_expression(s: &str) -> Expression {
    let (left, _, operation, _, right, _, target) = full(tuple((
        alphanumeric1,
        tag(" "),
        alphanumeric1,
        tag(" "),
        alphanumeric1,
        tag(" -> "),
        alphanumeric1,
    )))(s)
    .unwrap();
    Expression {
        left: Name(left.as_bytes().try_into().unwrap()),
        operation: match operation {
            "AND" => Operation::AND,
            "XOR" => Operation::XOR,
            "OR" => Operation::OR,
            _ => panic!("Unknown operation: {}", operation),
        },
        right: Name(right.as_bytes().try_into().unwrap()),
        target: Name(target.as_bytes().try_into().unwrap()),
    }
}

struct Day {
    starting_values: Vec<(Name, bool)>,
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
        if let Some(&(_, value)) = self.starting_values.iter().find(|(n, _)| n == name) {
            return Some(value);
        }

        let expression = self.expressions.get(name)?;
        let left = self.evaluate(&expression.left)?;
        let right = self.evaluate(&expression.right)?;
        match expression.operation {
            Operation::AND => Some(left & right),
            Operation::XOR => Some(left ^ right),
            Operation::OR => Some(left | right),
        }
    }

    fn detect_half_adder(&self, id: u16) -> Option<Name> {
        let target = Name::from(b'z', id);
        let from_x = Name::from(b'x', id);
        let from_y = Name::from(b'y', id);

        if let Some(expr) = self.expressions.get(&target) {
            if expr == &target.wired_from(from_x, Operation::XOR, from_y) {
                self.find_target_of_expression(&target.wired_from(from_x, Operation::AND, from_y))
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
        if source_target.is_none() || carry_in_expr.operation != Operation::XOR {
            let other_target = self
                .expressions
                .values()
                .find(|expr| {
                    expr.operation == Operation::XOR
                        && (expr.left == carry_in || expr.right == carry_in)
                })
                .unwrap()
                .target;

            return Right((target, other_target));
        }
        let source_target = *source_target.unwrap();

        let from_expr = self.find_expression(source_target).unwrap();
        let expected_expr = target.wired_from(from_x, Operation::XOR, from_y);
        if from_expr != &expected_expr {
            let other_target = self
                .expressions
                .values()
                .find(|expr| {
                    expr.operation == Operation::XOR
                        && (expr.left == from_x || expr.right == from_x)
                        && (expr.left == from_y || expr.right == from_y)
                })
                .unwrap()
                .target;

            return Right((source_target, other_target));
        }

        let carry_out_left = self
            .find_target_of_expression(&target.wired_from(source_target, Operation::AND, carry_in))
            .unwrap();

        let carry_out_right = self
            .find_target_of_expression(&target.wired_from(from_x, Operation::AND, from_y))
            .unwrap();

        Left(
            self.find_target_of_expression(&target.wired_from(
                carry_out_left,
                Operation::OR,
                carry_out_right,
            ))
            .unwrap(),
        )
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let starting_values = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|s| parse_name(&s))
            .collect();
        let expressions =
            lines.map(|s| parse_expression(&s)).map(|expr| (expr.target, expr)).collect();

        Day { starting_values, expressions }
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
