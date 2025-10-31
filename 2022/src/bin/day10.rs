#![feature(test)]
#![allow(clippy::ptr_arg)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from]
enum Command {
    #[format(preceded("addx ", i32))]
    Add(i32),
    #[format("noop")]
    Noop,
}

fn additions(commands: Vec<Command>) -> Vec<i32> {
    let mut additions = Vec::new();
    let mut x = 1i32;
    for command in commands {
        additions.push(x);
        match command {
            Command::Add(number) => {
                additions.push(x);
                x += number;
            }
            Command::Noop => {}
        }
    }
    additions
}

#[parse_from(map({}, additions))]
struct Additions(Vec<i32>);

fn calculate_part1(additions: &Additions) -> String {
    additions
        .0
        .iter()
        .enumerate()
        .filter(|(time, _)| time % 40 == 19)
        .map(|(time, x)| (time as i32 + 1) * x)
        .sum::<i32>()
        .to_string()
}

fn calculate_part2(additions: &Additions) -> String {
    let mut screen = String::with_capacity(256);
    additions.0.iter().enumerate().for_each(|(time, x)| {
        if (time % 40) == 0 {
            screen.push('\n');
        }
        screen.push(if (x - (time as i32 % 40)).abs() <= 1 { '#' } else { '.' });
    });
    screen
}

day_main!(Additions);
day_test!( 10, example => "13140".to_owned(), "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".to_owned() );
day_test!( 10 => "15260".to_owned(), "
###...##..#..#.####..##..#....#..#..##..
#..#.#..#.#..#.#....#..#.#....#..#.#..#.
#..#.#....####.###..#....#....#..#.#....
###..#.##.#..#.#....#.##.#....#..#.#.##.
#....#..#.#..#.#....#..#.#....#..#.#..#.
#.....###.#..#.#.....###.####..##...###.".to_owned() );
