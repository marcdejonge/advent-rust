use crate::parsing::{handle_parser_error, Parsable};
use memmap2::Mmap;
use num_format::{Locale, ToFormattedString};
use std::env;
use std::fmt::Display;
use std::time::Instant;

const FORMAT: Locale = Locale::en;

fn parse_input<Input>() -> Input
where
    Input: Parsable,
{
    let args: Vec<_> = env::args().collect();
    let file_name = args.get(1);
    if file_name.is_none() {
        println!("Please provide a file name as an argument");
        std::process::exit(1);
    }
    let file_name = file_name.unwrap();
    let file = std::fs::File::open(file_name).expect("Could not open file");
    let contents = unsafe { Mmap::map(&file).expect("Could not read file") };

    println!("Executing");

    let parse_file_start_time = Instant::now();
    let input: Input = handle_parser_error(&contents);

    println!(
        " ├── Input parsed \x1b[3min {}µs\x1b[0m",
        parse_file_start_time.elapsed().as_micros().to_formatted_string(&FORMAT)
    );

    input
}

fn execute_part<Input, Output, F>(name: &str, function: F, input: &Input)
where
    F: Fn(&Input) -> Output,
    Output: Display,
{
    let part1_calc_start_time = Instant::now();
    let part1 = function(input);
    println!(
        " ├── {} calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m",
        name,
        part1_calc_start_time.elapsed().as_micros().to_formatted_string(&FORMAT),
        part1
    );
}

pub fn execute_half_day<Input, O1, F1>(part1: F1)
where
    Input: Parsable,
    F1: Fn(&Input) -> O1,
    O1: Display,
{
    let before = Instant::now();
    let input = parse_input();
    execute_part("Part 1", part1, &input);

    println!(
        " └── Total time: \x1b[3m{}µs\x1b[0m",
        before.elapsed().as_micros().to_formatted_string(&FORMAT)
    );
    println!();
}

pub fn execute_day<ParseInput, FunctionInput, O1, O2, PF, F1, F2>(prepare: PF, part1: F1, part2: F2)
where
    ParseInput: Parsable,
    PF: Fn(ParseInput) -> FunctionInput,
    F1: Fn(&FunctionInput) -> O1,
    F2: Fn(&FunctionInput) -> O2,
    O1: Display,
    O2: Display,
{
    let before = Instant::now();
    let parsed_input = parse_input();

    let before_prepare = Instant::now();
    let input = prepare(parsed_input);
    let prepare_time = before_prepare.elapsed().as_micros();
    if prepare_time > 0 {
        println!(
            " ├── Preprocessed data \x1b[3min {}µs\x1b[0m",
            prepare_time.to_formatted_string(&FORMAT)
        );
    }

    execute_part("Part 1", part1, &input);
    execute_part("Part 2", part2, &input);

    println!(
        " └── Total time: \x1b[3m{}µs\x1b[0m",
        before.elapsed().as_micros().to_formatted_string(&FORMAT)
    );
    println!();
}

#[macro_export]
macro_rules! day_main {
    () => {
        day_main!(calculate_part1, calculate_part2);
    };
    ($part1:ident) => {
        fn main() { advent_lib::day::execute_half_day($part1) }
    };
    ($part1:ident, $part2:ident) => {
        fn main() { advent_lib::day::execute_day(std::convert::identity, $part1, $part2) }
    };
    ($prepare:path => $part1:ident, $part2:ident) => {
        fn main() { advent_lib::day::execute_day($prepare, $part1, $part2) }
    };
}
