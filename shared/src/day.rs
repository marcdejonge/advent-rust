use crate::parsing::handle_parser_error;
use memmap2::Mmap;
use nom::bytes::complete::take_while;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{AsBytes, Parser};
use num_format::{Locale, ToFormattedString};
use std::env;
use std::fmt::Display;
use std::time::Instant;

pub trait ExecutableDay
where
    Self: Sized,
{
    type Output;
    type AltOutput = Self::Output;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        map(
            separated_list1(line_ending, take_while(|b: u8| b != b'\n' && b != b'\r')),
            |lines: Vec<&[u8]>| {
                Self::from_lines(
                    lines.into_iter().map(|line| {
                        String::from_utf8(line.to_vec()).expect("Could not parse line")
                    }),
                )
            },
        )
    }
    fn from_lines<LINES: Iterator<Item = String>>(_lines: LINES) -> Self { todo!() }

    fn calculate_part1(&self) -> Self::Output;
    fn calculate_part2(&self) -> Self::AltOutput;
}

pub fn execute_day<Day: ExecutableDay>()
where
    Day::Output: Display,
    Day::AltOutput: Display,
{
    let args: Vec<_> = env::args().collect();
    let file_name = args.get(1);
    if file_name.is_none() {
        println!("Please provide a file name as an argument");
        return;
    }
    let file_name = file_name.unwrap();
    let file = std::fs::File::open(file_name).expect("Could not open file");
    let contents = unsafe { Mmap::map(&file).expect("Could not read file") };

    let format = Locale::en;
    println!("Executing");

    let parse_file_start_time = Instant::now();
    let day = handle_parser_error(contents.as_bytes(), Day::parser());

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
