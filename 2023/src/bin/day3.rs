#![feature(test)]

extern crate core;

use rayon::prelude::*;

use advent_lib::day::*;

struct Day {
    symbols: Vec<Symbol>,
    numbers: Vec<GridNumber>,
}

#[derive(Copy, Clone, Debug, Default)]
struct GridNumber {
    value: usize,
    length: i32,
    index: (i32, i32),
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Symbol {
    index: (i32, i32),
    is_gear: bool,
}

impl GridNumber {
    fn add_digit(&mut self, x: usize, y: usize, digit: u8) {
        if self.length == 0 {
            self.index = (x as i32, y as i32)
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
        (-1..=1).contains(&(symbol.index.1 - self.index.1))
            && (-1..=self.length).contains(&(symbol.index.0 - self.index.0))
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut numbers = Vec::<GridNumber>::new();
        let mut saved_number = GridNumber::default();
        let mut symbols = Vec::<Symbol>::new();

        lines.enumerate().for_each(|(y, line)| {
            line.bytes().enumerate().for_each(|(x, b)| match b {
                b'.' => saved_number.save(&mut numbers),
                b'0'..=b'9' => saved_number.add_digit(x, y, b),
                b'*' => {
                    saved_number.save(&mut numbers);
                    symbols.push(Symbol { index: (x as i32, y as i32), is_gear: true });
                }
                _ => {
                    saved_number.save(&mut numbers);
                    symbols.push(Symbol { index: (x as i32, y as i32), is_gear: false });
                }
            });
            saved_number.save(&mut numbers);
        });

        Day { symbols, numbers }
    }

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
