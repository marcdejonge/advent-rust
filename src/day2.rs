use crate::{ExecutableDay, execute_day};

pub(crate) fn execute() { execute_day::<Day2>(); }

struct Day2 {}

impl ExecutableDay for Day2 {
    type Input = Vec<String>;
    type Output = i32;

    fn get_code() -> i32 { 2 }

    fn parse_input(file_input: &str) -> Self::Input {
        file_input.lines().map(|line| line.to_owned()).collect()
    }

    fn calculate_part1(input: &Self::Input) -> Self::Output {
        input.iter().map(|line| match line.as_str() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("Unexpected game {}", line)
        }).sum()
    }

    fn calculate_part2(input: &Self::Input) -> Self::Output {
        input.iter().map(|line| match line.as_str() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!("Unexpected game {}", line)
        }).sum()
    }
}
