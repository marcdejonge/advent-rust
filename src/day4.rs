use std::ops::RangeInclusive;

use regex::Regex;

use crate::{ExecutableDay, execute_day};

pub(crate) fn execute() { execute_day::<Day>(); }

struct Day {}

impl ExecutableDay for Day {
    type Input = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;
    type Output = usize;

    fn get_code() -> i32 { 4 }

    fn parse_input(file_input: &str) -> Self::Input {
        let re = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
        file_input.lines().map(|line| {
            let cap = re.captures(line).unwrap();
            (
                RangeInclusive::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                RangeInclusive::new(cap[3].parse().unwrap(), cap[4].parse().unwrap())
            )
        }).collect()
    }

    fn calculate_part1(input: &Self::Input) -> Self::Output {
        input.iter().filter(|(first, second)| {
            is_contained(first, second) || is_contained(second, first)
        }).count()
    }

    fn calculate_part2(input: &Self::Input) -> Self::Output {
        input.iter().filter(|(first, second)| {
            first.contains(&second.start())
                || first.contains(&(second.end()))
                || second.contains(&first.start())
                || second.contains(&(first.end()))
        }).count()
    }
}

fn is_contained<T: PartialOrd<T>>(outside: &RangeInclusive<T>, inside: &RangeInclusive<T>) -> bool {
    outside.start() <= inside.start() && outside.end() >= inside.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = Day::parse_input("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(2, Day::calculate_part1(&input));
        assert_eq!(4, Day::calculate_part2(&input));
    }
}