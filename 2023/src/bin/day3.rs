#![feature(test)]

use advent_lib::grid::{Grid, Location};
use advent_lib::*;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[parse_from(map({}, |grid: Grid<char>| {
    let mut numbers = Vec::<GridNumber>::new();
    let mut saved_number = GridNumber::default();
    let mut symbols = Vec::<Symbol>::new();

    grid.east_lines().for_each(|line| {
        line.for_each(|(index, &b)| match b {
            '.' => saved_number.save(&mut numbers),
            '0'..='9' => saved_number.add_digit(index, b as u8),
            '*' => {
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
}))]
struct Input {
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

fn calculate_part1(input: &Input) -> usize {
    input
        .numbers
        .par_iter()
        .filter(|n| input.symbols.iter().any(|s| n.is_next_to(s)))
        .map(|n| n.value)
        .sum()
}

fn calculate_part2(input: &Input) -> usize {
    input
        .symbols
        .par_iter()
        .filter(|s| s.is_gear)
        .filter_map(|s| {
            let mut nrs = input.numbers.iter().filter(|n| n.is_next_to(s));
            Some(nrs.next()?.value * nrs.next()?.value)
        })
        .sum()
}

day_main!(Input);
day_test!( 3, example => 4361, 467835 );
day_test!( 3 => 536576, 75741499 );
