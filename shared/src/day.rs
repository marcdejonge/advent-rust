use crate::parsing::handle_parser_error;
use memmap2::Mmap;
use nom_parse_trait::ParseFrom;
use num_format::{Locale, ToFormattedString};
use std::env;
use std::path::Path;
use std::time::Instant;

const FORMAT: Locale = Locale::en;

pub fn format_time(instant: Instant) -> String {
    instant.elapsed().as_micros().to_formatted_string(&FORMAT)
}

pub fn parse_input<Input>() -> Input
where
    Input: for<'a> ParseFrom<&'a [u8]>,
{
    let args: Vec<_> = env::args().collect();
    let file_name = args.get(1).map(String::to_owned).unwrap_or_else(|| {
        let exec_path =
            Path::new(args.first().expect("Expected at least the name of the executable"));
        let file_name = exec_path.file_name().expect("Executable file can't be decoded");
        format!(
            "input/{}.txt",
            file_name.to_str().expect("Invalid file name")
        )
    });
    let file = std::fs::File::open(file_name).expect("Could not open input file");
    let contents = unsafe { Mmap::map(&file).expect("Could not read input file") };

    println!("Executing");

    let parse_file_start_time = Instant::now();
    let input: Input = handle_parser_error(&contents);

    println!(
        " ├── Input parsed \x1b[3min {}µs\x1b[0m",
        format_time(parse_file_start_time)
    );

    input
}

#[allow(clippy::crate_in_macro_def)] // This is the whole point, to use the call-site's crate
#[macro_export]
macro_rules! day_main_half {
    ($type:ty) => {
        type ParsedInput = $type;

        fn main() {
            let before = std::time::Instant::now();
            let input: ParsedInput = advent_lib::day::parse_input();

            let part1_start = std::time::Instant::now();
            let part1_output = crate::calculate_part1(&input);
            println!(
                " ├── Part 1 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m",
                advent_lib::day::format_time(part1_start),
                part1_output
            );

            println!(
                " └── Total time: \x1b[3m{}µs\x1b[0m",
                advent_lib::day::format_time(before)
            );
            println!();
        }
    };
}

#[allow(clippy::crate_in_macro_def)] // This is the whole point, to use the call-site's crate
#[macro_export]
macro_rules! day_main {
    ($type:ty) => {
        type ParsedInput = $type;

        fn main() {
            let before = std::time::Instant::now();
            let input: ParsedInput = advent_lib::day::parse_input();

            let part1_start = std::time::Instant::now();
            let part1_output = crate::calculate_part1(&input);
            println!(
                " ├── Part 1 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m",
                advent_lib::day::format_time(part1_start),
                part1_output
            );

            let part2_start = std::time::Instant::now();
            let part2_output = crate::calculate_part2(&input);
            println!(
                " ├── Part 2 calculated \x1b[3min {}µs\x1b[0m: \x1b[1m{}\x1b[0m",
                advent_lib::day::format_time(part2_start),
                part2_output
            );

            println!(
                " └── Total time: \x1b[3m{}µs\x1b[0m",
                advent_lib::day::format_time(before)
            );
            println!();
        }
    };
}
