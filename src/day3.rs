use std::collections::HashSet;

use crate::{ExecutableDay, execute_day};

pub(crate) fn execute() { execute_day::<Day>(); }

struct Day {}

impl ExecutableDay for Day {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn get_code() -> i32 { 3 }

    fn parse_input(file_input: &str) -> Self::Input {
        file_input.lines().map(|line| line.chars().map(|c| match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 27,
            _ => 0
        }).collect()).collect()
    }

    fn calculate_part1(input: &Self::Input) -> Self::Output {
        input.iter().map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let set: HashSet<_> = left.iter().copied().collect();
            right.iter().filter(|&c| set.contains(c)).last().unwrap()
        }).sum()
    }

    fn calculate_part2(input: &Self::Input) -> Self::Output {
        input.chunks(3).map(|lines| {
            let set = lines[0].iter().copied().collect::<HashSet<_>>()
                .intersection(&lines[1].iter().copied().collect())
                .copied().collect::<HashSet<_>>();
            lines[2].iter().filter(|&c| set.contains(c)).last().unwrap()
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = Day::parse_input("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(157, Day::calculate_part1(&input));
        assert_eq!(70, Day::calculate_part2(&input));
    }
}
