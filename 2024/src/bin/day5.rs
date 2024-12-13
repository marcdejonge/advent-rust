#![feature(test)]

use advent_lib::day::*;
use advent_lib::parsing::digits;
use fxhash::FxHashSet;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

struct Day {
    ordering_rules: FxHashSet<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

impl Day {
    fn calculate_middle(&self, use_changed: bool) -> u32 {
        self.pages
            .iter()
            .filter_map(|update| {
                let mut new = update.clone();
                new.sort_by(|&a, &b| {
                    if self.ordering_rules.contains(&(a, b)) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });

                if new.eq(update) ^ use_changed {
                    Some(new)
                } else {
                    None
                }
            })
            .map(|update| update[update.len() / 2])
            .sum()
    }
}

impl ExecutableDay for Day {
    type Output = u32;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let ordering_rules = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| separated_pair(digits, char('|'), digits)(&line).unwrap().1)
            .collect();

        let pages =
            lines.map(|line| separated_list1(char(','), digits)(&line).unwrap().1).collect();

        Day { ordering_rules, pages }
    }
    fn calculate_part1(&self) -> Self::Output { self.calculate_middle(false) }

    fn calculate_part2(&self) -> Self::Output { self.calculate_middle(true) }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 5, example1 => 143, 123 );
    day_test!( 5 => 5955, 4030);
}
