#![feature(test)]

use advent_lib::key::Key;
use advent_lib::parsing::{double_line_ending, separated_map1};
use advent_lib::*;
use fxhash::FxHashMap;
use itertools::Either::{Left, Right};
use itertools::{Either, Itertools};
use nom_parse_macros::parse_from;
use std::fmt::{Debug, Formatter};
use std::mem::swap;
use Operation::*;

#[parse_from]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Operation {
    #[format("AND")]
    And,
    #[format("XOR")]
    Xor,
    #[format("OR")]
    Or,
}

impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            And => write!(f, "AND"),
            Xor => write!(f, "XOR"),
            Or => write!(f, "OR"),
        }
    }
}

#[derive(Clone)]
#[parse_from((
    Key::parse,
    delimited(space0, Operation::parse, space0),
    Key::parse,
    preceded(" -> ", Key::parse),
))]
struct Expression {
    left: Key,
    operation: Operation,
    right: Key,
    target: Key,
}

impl Expression {
    fn get_other_input(&self, name: Key) -> Option<Key> {
        if self.left == name {
            Some(self.right)
        } else if self.right == name {
            Some(self.left)
        } else {
            None
        }
    }
}

impl Expression {
    fn new(left: Key, operation: Operation, right: Key, target: Key) -> Self {
        Self { left, operation, right, target }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation
            && ((self.left == other.left && self.right == other.right)
                || (self.left == other.right && self.right == other.left))
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?} {} -> {}",
            self.left, self.operation, self.right, self.target
        )
    }
}

#[parse_from(separated_pair(
    separated_map1(
        line_ending,
        separated_pair(Key::parse, ": ", map(u8, |v| v != 0))
    ),
    double_line_ending,
    separated_map1(
        line_ending,
        map(Expression::parse, |expr| (expr.target, expr))
    ),
))]
struct Computer {
    starting_values: FxHashMap<Key, bool>,
    expressions: FxHashMap<Key, Expression>,
}

const X: Key = Key::fixed(b"x00");
const Y: Key = Key::fixed(b"y00");
const Z: Key = Key::fixed(b"z00");

fn create_key(from: Key, id: usize) -> Key { from + (id / 10) * 36 + (id % 10) }

impl Computer {
    fn find_target_of_expression(&self, expected: &Expression) -> Option<Key> {
        let result = self.expressions.iter().find(|(_, expr)| expr == &expected).map(|(n, _)| *n);
        if result.is_none() {
            println!("Could not find expression: {:?}", expected);
        }
        result
    }

    fn find_expression(&self, target: Key) -> Option<&Expression> {
        let result = self.expressions.get(&target);
        if result.is_none() {
            println!("Could not find expression for target: {}", target);
        }
        result
    }

    fn evaluate(&self, name: Key) -> Option<bool> {
        if let Some(&value) = self.starting_values.get(&name) {
            return Some(value);
        }

        let expression = self.expressions.get(&name)?;
        let left = self.evaluate(expression.left)?;
        let right = self.evaluate(expression.right)?;
        match expression.operation {
            And => Some(left & right),
            Xor => Some(left ^ right),
            Or => Some(left | right),
        }
    }

    fn detect_half_adder(&self, id: usize) -> Option<Key> {
        let from_x = create_key(X, id);
        let from_y = create_key(Y, id);
        let target = create_key(Z, id);

        if let Some(expr) = self.expressions.get(&target) {
            if expr == &Expression::new(from_x, Xor, from_y, target) {
                self.find_target_of_expression(&Expression::new(from_x, And, from_y, target))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn detect_full_adder(&self, id: usize, carry_in: Key) -> Either<Key, (Key, Key)> {
        let from_x = create_key(X, id);
        let from_y = create_key(Y, id);
        let target = create_key(Z, id);

        let carry_in_expr = self.find_expression(target).unwrap();
        let source_target = carry_in_expr.get_other_input(carry_in);
        if source_target.is_none() || carry_in_expr.operation != Xor {
            let other_target = self
                .expressions
                .values()
                .find(|expr| {
                    expr.operation == Xor && (expr.left == carry_in || expr.right == carry_in)
                })
                .unwrap()
                .target;

            return Right((target, other_target));
        }
        let source_target = source_target.unwrap();

        let from_expr = self.find_expression(source_target).unwrap();
        let expected_expr = Expression::new(from_x, Xor, from_y, target);
        if from_expr != &expected_expr {
            let other_target = self
                .expressions
                .values()
                .find(|expr| {
                    expr.operation == Xor
                        && (expr.left == from_x || expr.right == from_x)
                        && (expr.left == from_y || expr.right == from_y)
                })
                .unwrap()
                .target;

            return Right((source_target, other_target));
        }

        let carry_left = self
            .find_target_of_expression(&Expression::new(source_target, And, carry_in, target))
            .unwrap();

        let carry_right = self
            .find_target_of_expression(&Expression::new(from_x, And, from_y, target))
            .unwrap();

        Left(
            self.find_target_of_expression(&Expression::new(carry_left, Or, carry_right, target))
                .unwrap(),
        )
    }
}

fn calculate_part1(computer: &Computer) -> u64 {
    let mut result = 0;
    for ix in 0..64 {
        let name = create_key(Z, ix);
        if !computer.expressions.contains_key(&name) {
            break;
        }

        if let Some(value) = computer.evaluate(name) {
            if value {
                result |= 1 << ix;
            }
        }
    }

    result
}
fn calculate_part2(computer: &Computer) -> String {
    let mut fixed_day = Computer {
        starting_values: computer.starting_values.clone(),
        expressions: computer.expressions.clone(),
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
    swapped_names.iter().map(|n| n.to_string()).join(",")
}

day_main!(Computer);
day_test!( 24, example1 => 4 );
day_test!( 24, example2 => 2024 );
day_test!( 24 => 48508229772400, "cqr,ncd,nfj,qnw,vkg,z15,z20,z37".to_string() );
