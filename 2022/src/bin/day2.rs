#![feature(test)]
use advent_lib::day::*;

struct Day {
    input: Vec<String>,
}

impl ExecutableDay for Day {
    type Output = i32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { input: lines.collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.input
            .iter()
            .map(|line| match line.as_str() {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _ => panic!("Unexpected game {}", line),
            })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.input
            .iter()
            .map(|line| match line.as_str() {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                _ => panic!("Unexpected game {}", line),
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 2, example => 15, 12 );
    day_test!( 2 => 13565, 12424 );
}
