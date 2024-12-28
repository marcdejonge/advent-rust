#![feature(test)]

use advent_lib::day::*;
use advent_lib::key::Key;
use advent_macros::parsable;
use fxhash::FxHashMap;
use std::ops::RangeInclusive;
use CheckType::*;

#[parsable(
    in_braces(map(
        tuple((preceded(tag(b"x="), i64),preceded(tag(b",m="), i64),preceded(tag(b",a="), i64),preceded(tag(b",s="), i64))),
        |(x, m, a, s)| [x, m, a, s],
    ))
)]
struct Parts([i64; 4]);
type PartsRange = [RangeInclusive<i64>; 4];

#[parsable(
    separated_pair(
        separated_map1(
            line_ending,
            tuple((
                Key::parser(),
                in_braces(
                    tuple((
                        separated_list1(tag(b","), Check::parser()),
                        preceded(tag(b","), Key::parser()),
                    ))
                )
            )),
        ),
        double_line_ending,
        separated_lines1(),
    )
)]
struct Day {
    checks: FxHashMap<Key, (Vec<Check>, Key)>,
    parts: Vec<Parts>,
}

const R: Key = Key::fixed(b"R");
const A: Key = Key::fixed(b"A");
const IN: Key = Key::fixed(b"in");

impl Day {
    fn calculate(&self, key: Key, parts: &Parts) -> i64 {
        if key == R {
            return 0;
        } else if key == A {
            return parts.0.iter().sum();
        }

        let (checks, other) = self.checks.get(&key).expect("Cannot find check");
        for check in checks {
            let count = parts.0[check.type_key];

            match check.check_type {
                LessThan if count < check.size => return self.calculate(check.to, parts),
                GreaterThan if count > check.size => return self.calculate(check.to, parts),
                _ => {}
            }
        }

        self.calculate(*other, parts)
    }

    fn calculate_range(&self, key: Key, parts_range: PartsRange) -> i64 {
        if key == R {
            return 0;
        } else if key == A {
            return parts_range.into_iter().map(|range| range.end() - range.start() + 1).product();
        }

        let (checks, other) = self.checks.get(&key).expect("Cannot find check");
        let mut result = 0;

        let mut range_left = parts_range;
        for check in checks {
            let start = *range_left[check.type_key].start();
            let end = *range_left[check.type_key].end();

            match check.check_type {
                LessThan if end < check.size => {
                    return result + self.calculate_range(check.to, range_left)
                }
                GreaterThan if start > check.size => {
                    return result + self.calculate_range(check.to, range_left)
                }
                LessThan if start < check.size => {
                    let mut split_range = range_left.clone();
                    split_range[check.type_key] = start..=(check.size - 1);
                    range_left[check.type_key] = check.size..=end;
                    result += self.calculate_range(check.to, split_range);
                }
                GreaterThan if end > check.size => {
                    let mut split_range = range_left.clone();
                    range_left[check.type_key] = start..=check.size;
                    split_range[check.type_key] = (check.size + 1)..=end;
                    result += self.calculate_range(check.to, split_range);
                }
                _ => {}
            }
        }

        result + self.calculate_range(*other, range_left)
    }
}

#[parsable(
    tuple((
        alt((
            value(0, tag(b"x")),
            value(1, tag(b"m")),
            value(2, tag(b"a")),
            value(3, tag(b"s")),
        )),
        alt((
            value(LessThan, tag(b"<")),
            value(GreaterThan, tag(b">")),
        )),
        i64,
        preceded(
            tag(b":"),
            Key::parser(),
        ),
    ))
)]
struct Check {
    type_key: usize,
    check_type: CheckType,
    size: i64,
    to: Key,
}

#[derive(Clone)]
enum CheckType {
    LessThan,
    GreaterThan,
}

impl ExecutableDay for Day {
    type Output = i64;

    fn calculate_part1(&self) -> Self::Output {
        self.parts.iter().map(|parts| self.calculate(IN, parts)).sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.calculate_range(IN, [1..=4000, 1..=4000, 1..=4000, 1..=4000])
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 19, example => 19114, 167409079868000 );
    day_test!( 19 => 425811, 131796824371749 );
}
