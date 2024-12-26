#![feature(test)]

use advent_lib::day::*;
use advent_lib::grid::Grid;
use advent_lib::parsing::double_line_parser;
use nom::combinator::map;
use nom::error::Error;
use nom::Parser;

#[derive(Debug)]
struct Day {
    locks: Vec<Vec<usize>>,
    keys: Vec<Vec<usize>>,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(double_line_parser(), |grids: Vec<Grid<u8>>| {
            let mut locks = Vec::new();
            let mut keys = Vec::new();

            for grid in grids {
                if grid.east_line(0).all(|(_, &b)| b == b'#') {
                    locks.push(
                        grid.x_range()
                            .map(|x| grid.south_line(x).take_while(|(_, &b)| b == b'#').count())
                            .collect(),
                    )
                } else if grid.east_line(grid.height() - 1).all(|(_, &b)| b == b'#') {
                    keys.push(
                        grid.x_range()
                            .map(|x| grid.north_line(x).take_while(|(_, &b)| b == b'#').count())
                            .collect(),
                    )
                } else {
                    println!("WARN unexpected grid: \n{:?}", grid);
                }
            }

            Day { locks, keys }
        })
    }

    fn calculate_part1(&self) -> Self::Output {
        self.keys
            .iter()
            .flat_map(|key| self.locks.iter().map(move |lock| (key, lock)))
            .filter(|&(key, lock)| (0..key.len()).all(|index| key[index] + lock[index] <= 7))
            .count()
    }
    fn calculate_part2(&self) -> Self::Output { 0 }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 25, example1 => 3 );
    day_test!( 25 => 3021 );
}
