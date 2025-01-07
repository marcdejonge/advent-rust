#![feature(test)]

use advent_lib::day_main;
use advent_lib::parsing::{double_line_ending, separated_set1};
use fxhash::FxHashSet;
use nom_parse_macros::parse_from;

#[parse_from(separated_pair(
    separated_set1::<I, E, _, _>(line_ending, separated_pair(u32, "|", u32)),
    double_line_ending,
    separated_list1(line_ending, separated_list1(",", u32)),
))]
struct Almanac {
    ordering_rules: FxHashSet<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

impl Almanac {
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

fn calculate_part1(almanac: &Almanac) -> u32 { almanac.calculate_middle(false) }

fn calculate_part2(almanac: &Almanac) -> u32 { almanac.calculate_middle(true) }

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 5, example1 => 143, 123 );
    day_test!( 5 => 5955, 4030);
}
