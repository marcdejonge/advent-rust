#![feature(test)]
#![feature(const_for)]

use advent_lib::day::{execute_day, ExecutableDay};
use fxhash::FxBuildHasher;
use std::collections::HashMap;

type Monkeys = HashMap<Name, Monkey, FxBuildHasher>;
type Number = i64;

struct Day {
    root_name: Name,
    monkeys: Monkeys,
}

fn solve(monkeys: &Monkeys, name: &Name) -> Value {
    macro_rules! solve {
            ($left:ident, $right:ident => $operation: tt $type_left: tt $type_right: tt) => {
                match (solve(monkeys, $left), solve(monkeys, $right)) {
                    (Value::Unknown { .. }, Value::Unknown { .. }) => {
                        panic!("Both sides are unknown, cannot solve")
                    }
                    (Value::Constant { value: left }, Value::Constant { value: right }) => {
                        Value::Constant { value: left $operation right }
                    }
                    (Value::Constant { value: left }, Value::Unknown { mut applications }) => {
                        applications.push(Application::$type_left { value: left });
                        Value::Unknown { applications }
                    }
                    (Value::Unknown { mut applications }, Value::Constant { value: right }) => {
                        applications.push(Application::$type_right { value: right });
                        Value::Unknown { applications }
                    }
                }
            };
        }

    match monkeys.get(name).expect("Could not find monkey") {
        Monkey::Unknown => Value::Unknown { applications: Vec::with_capacity(100) },
        Monkey::Constant { value } => Value::Constant { value: *value },
        Monkey::Add { left, right } => solve!(left, right => + Add Add),
        Monkey::Subtract { left, right } => solve!(left, right => - SubtractFrom SubtractOf),
        Monkey::Multiply { left, right } => solve!(left, right => * Multiply Multiply),
        Monkey::Divide { left, right } => solve!(left, right => / DivisionOf DivideBy),
        Monkey::Equal { left, right } => match (solve(monkeys, left), solve(monkeys, right)) {
            (Value::Unknown { .. }, Value::Unknown { .. }) => {
                panic!("Both sides are unknown, cannot solve")
            }
            (Value::Constant { .. }, Value::Constant { .. }) => {
                panic!("Both sides are constant, cannot solve")
            }
            (Value::Unknown { applications }, Value::Constant { value }) => Value::Constant {
                value: applications.iter().rev().fold(value, |acc, a| a.reverse(acc)),
            },
            (Value::Constant { value }, Value::Unknown { applications }) => Value::Constant {
                value: applications.iter().rev().fold(value, |acc, a| a.reverse(acc)),
            },
        },
    }
}

type Name = u32;

#[derive(Clone, Debug)]
enum Monkey {
    Unknown,
    Constant { value: Number },
    Add { left: Name, right: Name },
    Subtract { left: Name, right: Name },
    Multiply { left: Name, right: Name },
    Divide { left: Name, right: Name },
    Equal { left: Name, right: Name },
}

enum Value {
    Unknown { applications: Vec<Application> },
    Constant { value: Number },
}

enum Application {
    Add { value: Number },
    SubtractFrom { value: Number },
    SubtractOf { value: Number },
    Multiply { value: Number },
    DivideBy { value: Number },
    DivisionOf { value: Number },
}

impl Application {
    fn reverse(&self, result: Number) -> Number {
        match self {
            Application::Add { value } => result - value,
            Application::SubtractFrom { value } => value - result,
            Application::SubtractOf { value } => result + value,
            Application::Multiply { value } => result / value,
            Application::DivideBy { value } => result * value,
            Application::DivisionOf { value } => value / result,
        }
    }
}

fn parse_name(name: &str) -> u32 {
    name.as_bytes().iter().fold(0, |acc, b| 26 * acc + (b - b'a') as u32)
}

impl ExecutableDay for Day {
    type Output = Number;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut monkeys = HashMap::with_capacity_and_hasher(5000, FxBuildHasher::default());
        for line in lines {
            let mut parts = line.split(' ');
            let monkey_name =
                parse_name(parts.next().expect("Expected monkey name").trim_end_matches(":"));
            let left = parts.next().expect("Expect first part");
            let operation = parts.next();
            if let Some(operation) = operation {
                let left = parse_name(left);
                let right = parse_name(parts.next().expect("Expected second part"));
                monkeys.insert(
                    monkey_name,
                    match operation {
                        "+" => Monkey::Add { left, right },
                        "-" => Monkey::Subtract { left, right },
                        "*" => Monkey::Multiply { left, right },
                        "/" => Monkey::Divide { left, right },
                        _ => panic!("Unknown operation"),
                    },
                );
            } else {
                monkeys.insert(
                    monkey_name,
                    Monkey::Constant { value: left.parse().expect("Expected constant number") },
                );
            }
        }
        Day { root_name: parse_name("root"), monkeys }
    }

    fn calculate_part1(&self) -> Self::Output {
        if let Value::Constant { value } = solve(&self.monkeys, &self.root_name) {
            value
        } else {
            panic!("Could not find a solution")
        }
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut monkeys = self.monkeys.clone();
        if let Monkey::Add { left, right } = monkeys.get(&self.root_name).unwrap() {
            monkeys.insert(self.root_name, Monkey::Equal { left: *left, right: *right });
        }
        monkeys.insert(parse_name("humn"), Monkey::Unknown);

        if let Value::Constant { value } = solve(&monkeys, &self.root_name) {
            value
        } else {
            panic!("Could not find a solution")
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 21, example => 152, 301 );
    day_test!( 21 => 21208142603224, 3882224466191 );
}
