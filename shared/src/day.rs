use num_format::{Locale, ToFormattedString};
use std::fmt::Display;
use std::io::{stdin, BufRead};
use std::time::Instant;

pub trait ExecutableDay {
    type Output;

    fn calculate_part1(&self) -> Self::Output;
    fn calculate_part2(&self) -> Self::Output;
}

pub fn execute_day<Day>()
where
    Day: ExecutableDay + FromIterator<String>,
    Day::Output: Display,
{
    let format = Locale::en;
    println!("Executing");

    let parse_file_start_time = Instant::now();
    let day: Day = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read from input"))
        .collect();
    println!(
        " ├── Input parsed \x1b[3min {}µs\x1b[0m",
        parse_file_start_time.elapsed().as_micros().to_formatted_string(&format)
    );

    let part1_calc_start_time = Instant::now();
    let part1 = day.calculate_part1();
    println!(
        " ├── Part 1 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m",
        part1_calc_start_time.elapsed().as_micros().to_formatted_string(&format),
        part1
    );

    let part2_calc_start_time = Instant::now();
    let part2 = day.calculate_part2();
    println!(
        " ├── Part 2 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m",
        part2_calc_start_time.elapsed().as_micros().to_formatted_string(&format),
        part2
    );

    println!(
        " └── Total time: \x1b[3m{}µs\x1b[0m",
        parse_file_start_time.elapsed().as_micros().to_formatted_string(&format)
    );
    println!();
}
