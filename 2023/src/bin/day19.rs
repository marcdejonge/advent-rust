#![feature(test)]

use std::ops::RangeInclusive;

use fxhash::FxHashMap;
use prse::*;

use advent_lib::day::*;
use CheckType::*;

type Number = i64;

type Parts = [Number; 4];
type PartsRange = [RangeInclusive<Number>; 4];

struct Day {
    checks: FxHashMap<String, (Vec<Check>, String)>,
    parts: Vec<Parts>,
}

impl Day {
    fn calculate(&self, key: &str, parts: &Parts) -> Number {
        if key == "R" {
            return 0;
        } else if key == "A" {
            return parts.iter().sum();
        }

        let (checks, other) = self.checks.get(key).expect("Cannot find check");
        for check in checks {
            let count = parts[check.type_key];

            match check.check_type {
                LessThan if count < check.size => return self.calculate(&check.to, parts),
                GreaterThan if count > check.size => return self.calculate(&check.to, parts),
                _ => {}
            }
        }

        self.calculate(other, parts)
    }

    fn calculate_range(&self, key: &str, parts_range: PartsRange) -> Number {
        if key == "R" {
            return 0;
        } else if key == "A" {
            return parts_range.into_iter().map(|range| range.end() - range.start() + 1).product();
        }

        let (checks, other) = self.checks.get(key).expect("Cannot find check");
        let mut result = 0;

        let mut range_left = parts_range;
        for check in checks {
            let start = *range_left[check.type_key].start();
            let end = *range_left[check.type_key].end();

            match check.check_type {
                LessThan if end < check.size => {
                    return result + self.calculate_range(&check.to, range_left)
                }
                GreaterThan if start > check.size => {
                    return result + self.calculate_range(&check.to, range_left)
                }
                LessThan if start < check.size => {
                    let mut split_range = range_left.clone();
                    split_range[check.type_key] = start..=(check.size - 1);
                    range_left[check.type_key] = check.size..=end;
                    result += self.calculate_range(&check.to, split_range);
                }
                GreaterThan if end > check.size => {
                    let mut split_range = range_left.clone();
                    range_left[check.type_key] = start..=check.size;
                    split_range[check.type_key] = (check.size + 1)..=end;
                    result += self.calculate_range(&check.to, split_range);
                }
                _ => {}
            }
        }

        result + self.calculate_range(other, range_left)
    }
}

struct Check {
    type_key: usize,
    check_type: CheckType,
    size: Number,
    to: String,
}

enum CheckType {
    LessThan,
    GreaterThan,
}

impl<'a> Parse<'a> for Check {
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if s.len() < 5 {
            return Err(ParseError::new("String too short"));
        }
        let colon = s.find(':').ok_or(ParseError::new("Missing colon"))?;
        let size = Number::from_str(&s[2..colon])?;
        let to = s[colon + 1..].to_string();
        let (type_key, less_than) = match &s[0..2] {
            "x<" => (0, LessThan),
            "x>" => (0, GreaterThan),
            "m<" => (1, LessThan),
            "m>" => (1, GreaterThan),
            "a<" => (2, LessThan),
            "a>" => (2, GreaterThan),
            "s<" => (3, LessThan),
            "s>" => (3, GreaterThan),
            _ => Err(ParseError::new("Invalid check"))?,
        };
        Ok(Check { type_key, check_type: less_than, size, to })
    }
}

impl ExecutableDay for Day {
    type Output = Number;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let checks = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (key, mut checks): (String, Vec<String>) = parse!(line, "{}{{{:,:}}}");
                let other = checks.pop().unwrap();
                let checks = checks.iter().map(|s| Check::from_str(s).unwrap()).collect();
                (key, (checks, other))
            })
            .collect();
        let parts = lines.map(|line| parse!(line, "{{x={},m={},a={},s={}}}").into()).collect();
        Day { checks, parts }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.parts.iter().map(|parts| self.calculate("in", parts)).sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.calculate_range("in", [1..=4000, 1..=4000, 1..=4000, 1..=4000])
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 19, example => 19114, 167409079868000 );
    day_test!( 19 => 425811, 131796824371749 );
}
