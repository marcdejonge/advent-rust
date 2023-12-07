/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up
to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't
expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix
it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure
out which one. If you can add up all the part numbers in the engine schematic, it should be easy to
work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There
are lots of numbers and symbols you don't really understand, but apparently any number adjacent to
a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do
not count as a symbol.)

What is the sum of all of the part numbers in the engine schematic?

--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life,
you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the
gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the
engineer, holding a phone in one hand and waving with the other. You're going so slowly that you
haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any *
symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying
those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer
can figure out which gear needs to be replaced.

What is the sum of all of the gear ratios in your engine schematic?
*/

#![feature(test)]

extern crate core;

use advent_lib::day::{execute_day, ExecutableDay};

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
            .iter()
            .filter(|n| self.symbols.iter().any(|s| n.is_next_to(s)))
            .map(|n| n.value)
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.symbols
            .iter()
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
