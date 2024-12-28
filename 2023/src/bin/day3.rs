#![feature(test)]

extern crate core;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::grid::{Grid, Location};
use advent_macros::parsable;
use rayon::prelude::*;

#[parsable(
    map_parsable(|grid: Grid<_>| {
        let mut numbers = Vec::<GridNumber>::new();
        let mut saved_number = GridNumber::default();
        let mut symbols = Vec::<Symbol>::new();

        grid.east_lines().for_each(|line| {
            line.for_each(|(index, &b)| match b {
                b'.' => saved_number.save(&mut numbers),
                b'0'..=b'9' => saved_number.add_digit(index, b),
                b'*' => {
                    saved_number.save(&mut numbers);
                    symbols.push(Symbol { index, is_gear: true });
                }
                _ => {
                    saved_number.save(&mut numbers);
                    symbols.push(Symbol { index, is_gear: false });
                }
            });
            saved_number.save(&mut numbers);
        });
        (symbols, numbers)
    })
)]
struct Day {
    symbols: Vec<Symbol>,
    numbers: Vec<GridNumber>,
}

#[derive(Copy, Clone, Debug, Default)]
struct GridNumber {
    value: usize,
    length: i32,
    index: Location,
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Symbol {
    index: Location,
    is_gear: bool,
}

impl GridNumber {
    fn add_digit(&mut self, location: Location, digit: u8) {
        if self.length == 0 {
            self.index = location
        }
        self.value = self.value * 10 + (digit - b'0') as usize;
        self.length += 1;
    }

    fn save(&mut self, numbers: &mut Vec<GridNumber>) {
        if self.length > 0 {
            numbers.push(*self);
            self.length = 0;
            self.value = 0;
        }
    }

    fn is_next_to(&self, symbol: &Symbol) -> bool {
        (-1..=1).contains(&(symbol.index.y() - self.index.y()))
            && (-1..=self.length).contains(&(symbol.index.x() - self.index.x()))
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn calculate_part1(&self) -> Self::Output {
        self.numbers
            .par_iter()
            .filter(|n| self.symbols.iter().any(|s| n.is_next_to(s)))
            .map(|n| n.value)
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.symbols
            .par_iter()
            .filter(|s| s.is_gear)
            .filter_map(|s| {
                let mut nrs = self.numbers.iter().filter(|n| n.is_next_to(s));
                Some(nrs.next()?.value * nrs.next()?.value)
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 3, example => 4361, 467835 );
    day_test!( 3 => 536576, 75741499 );
}
