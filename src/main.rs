use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;

use clap::Parser;
use num_format::{Locale, ToFormattedString};

mod iter_utils;

mod day1;
mod day2;
mod day3;
mod day4;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: usize,
}

trait ExecutableDay {
    type Input;
    type Output: Display;

    fn get_code() -> i32;
    fn parse_input(file_input: &str) -> Self::Input;
    fn calculate_part1(input: &Self::Input) -> Self::Output;
    fn calculate_part2(input: &Self::Input) -> Self::Output;
}

fn load_file<T: ExecutableDay>() -> String {
    let file_name = format!("input/day{:02}.txt", T::get_code());
    let error = format!("Could not find file {}", file_name);
    read_to_string(file_name).expect(&error)
}

fn execute_day<T: ExecutableDay>() {
    let format = Locale::en;
    println!("Executing Day {}", T::get_code());

    let file_load_start_time = Instant::now();
    let file_contents = load_file::<T>();
    println!(" ├── File loaded \x1b[3min {}µs\x1b[0m", file_load_start_time.elapsed().as_micros().to_formatted_string(&format));

    let parse_file_start_time = Instant::now();
    let input = T::parse_input(&file_contents);
    println!(" ├── Input parsed \x1b[3min {}µs\x1b[0m", parse_file_start_time.elapsed().as_micros().to_formatted_string(&format));

    let part1_calc_start_time = Instant::now();
    let part1 = T::calculate_part1(&input);
    println!(" ├── Part 1 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m", part1_calc_start_time.elapsed().as_micros().to_formatted_string(&format), part1);

    let part2_calc_start_time = Instant::now();
    let part2 = T::calculate_part2(&input);
    println!(" ├── Part 2 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m", part2_calc_start_time.elapsed().as_micros().to_formatted_string(&format), part2);

    println!(" └── Total time: \x1b[3m{}µs\x1b[0m", file_load_start_time.elapsed().as_micros().to_formatted_string(&format));
    println!();
}

macro_rules! days {
    ( $day: expr, $( $x: ident), * ) => {{
        let mut index = 1..;
        if($day == 0) {
            $($x::execute();)*
        } else $(
            if($day == index.next().unwrap()) {
                $x::execute();
            } else
        )* {
            println!("Day {} has not been implemented, only 1 to {} are valid", $day, index.next().unwrap() - 1);
        }
    }};
}

fn main() {
    let args = Args::parse();
    days!(args.day, day1, day2, day3, day4)
}
