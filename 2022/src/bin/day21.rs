#![feature(test)]
#![feature(const_for)]

use advent_lib::day_main;
use advent_lib::key::Key;
use advent_lib::parsing::separated_map1;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;

#[derive(Clone, Debug)]
#[parse_from(separated_map1(line_ending, separated_pair(Key::parse, ": ", Monkey::parse)))]
struct Monkeys(FxHashMap<Key, Monkey>);
type Number = i64;
const ROOT: Key = Key::fixed(b"root");

fn solve(monkeys: &Monkeys, name: Key) -> Value {
    macro_rules! solve {
            ($left:ident, $right:ident => $operation: tt $type_left: tt $type_right: tt) => {
                match (solve(monkeys, *$left), solve(monkeys, *$right)) {
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

    match monkeys.0.get(&name).expect("Could not find monkey") {
        Monkey::Unknown => Value::Unknown { applications: Vec::with_capacity(100) },
        Monkey::Constant { value, .. } => Value::Constant { value: *value },
        Monkey::Add { left, right, .. } => solve!(left, right => + Add Add),
        Monkey::Subtract { left, right, .. } => solve!(left, right => - SubtractFrom SubtractOf),
        Monkey::Multiply { left, right, .. } => solve!(left, right => * Multiply Multiply),
        Monkey::Divide { left, right, .. } => solve!(left, right => / DivisionOf DivideBy),
        Monkey::Equal { left, right, .. } => {
            match (solve(monkeys, *left), solve(monkeys, *right)) {
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
            }
        }
    }
}

#[derive(Clone, Debug)]
#[parse_from]
enum Monkey {
    #[format(fail::<_, (), _>)]
    Unknown,
    #[format(i64)]
    Constant { value: Number },
    #[format(separated_pair(Key::parse, " + ", Key::parse))]
    Add { left: Key, right: Key },
    #[format(separated_pair(Key::parse, " - ", Key::parse))]
    Subtract { left: Key, right: Key },
    #[format(separated_pair(Key::parse, " * ", Key::parse))]
    Multiply { left: Key, right: Key },
    #[format(separated_pair(Key::parse, " / ", Key::parse))]
    Divide { left: Key, right: Key },
    #[format(separated_pair(Key::parse, " = ", Key::parse))]
    Equal { left: Key, right: Key },
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

fn calculate_part1(monkeys: &Monkeys) -> i64 {
    if let Value::Constant { value } = solve(&monkeys, ROOT) {
        value
    } else {
        panic!("Could not find a solution")
    }
}

fn calculate_part2(monkeys: &Monkeys) -> i64 {
    let mut monkeys = monkeys.clone();
    if let Monkey::Add { left, right } = monkeys.0.get(&ROOT).cloned().unwrap() {
        monkeys.0.insert(ROOT, Monkey::Equal { left, right });
    }
    monkeys.0.insert(Key::fixed(b"humn"), Monkey::Unknown);

    if let Value::Constant { value } = solve(&monkeys, ROOT) {
        value
    } else {
        panic!("Could not find a solution")
    }
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 21, example => 152, 301 );
    day_test!( 21 => 21208142603224, 3882224466191 );
}
