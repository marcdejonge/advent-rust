use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;

use clap::Parser;

mod iter_utils;

mod day1;
mod day2;

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
    println!();
    println!("Executing Day {}", T::get_code());

    let file_load_start_time = Instant::now();
    let file_contents = load_file::<T>();
    println!(" ├── File loaded in {}ms", file_load_start_time.elapsed().as_millis());

    let parse_file_start_time = Instant::now();
    let input = T::parse_input(&file_contents);
    println!(" ├── Input parsed in in {}ms", parse_file_start_time.elapsed().as_millis());

    let part1_calc_start_time = Instant::now();
    let part1 = T::calculate_part1(&input);
    println!(" ├── Part 1 calculated in {}ms: {}", part1_calc_start_time.elapsed().as_millis(), part1);

    let part2_calc_start_time = Instant::now();
    let part2 = T::calculate_part2(&input);
    println!(" ├── Part 2 calculated in {}ms: {}", part2_calc_start_time.elapsed().as_millis(), part2);

    println!(" └── Total time: {}ms", file_load_start_time.elapsed().as_millis());
}

fn main() {
    let args = Args::parse();

    match args.day {
        0 => {
            day1::execute();
            day2::execute();
        }
        1 => day1::execute(),
        2 => day2::execute(),
        _ => { println!("Day {} has not been implemented, only 1 to 2 are valid", args.day) }
    }
}
