#![feature(test)]

use advent_lib::day_main;
use advent_macros::parsable;

#[parsable]
enum Command {
    #[format=preceded(tag(b"addx "), i32)]
    Add(i32),
    #[format=tag(b"noop")]
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

fn calculate_part1(additions: &Vec<i32>) -> String {
    additions
        .iter()
        .enumerate()
        .filter(|(time, _)| time % 40 == 19)
        .map(|(time, x)| (time as i32 + 1) * x)
        .sum::<i32>()
        .to_string()
}

fn calculate_part2(additions: &Vec<i32>) -> String {
    let mut screen = String::with_capacity(256);
    additions.iter().enumerate().for_each(|(time, x)| {
        if (time % 40) == 0 {
            screen.push('\n');
        }
        screen.push(if (x - (time as i32 % 40)).abs() <= 1 { '#' } else { '.' });
    });
    screen
}

day_main!( additions => calculate_part1, calculate_part2 );

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 10, example => "13140".to_owned(), "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".to_owned() ; additions );
    day_test!( 10 => "15260".to_owned(), "
###...##..#..#.####..##..#....#..#..##..
#..#.#..#.#..#.#....#..#.#....#..#.#..#.
#..#.#....####.###..#....#....#..#.#....
###..#.##.#..#.#....#.##.#....#..#.#.##.
#....#..#.#..#.#....#..#.#....#..#.#..#.
#.....###.#..#.#.....###.####..##...###.".to_owned() ; additions );
}
