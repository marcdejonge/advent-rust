#![feature(test)]

use advent_lib::{parsing::double_line_ending, *};
use nom_parse_macros::parse_from;
use std::cmp::Ordering::*;

#[parse_from(separated_pair(
    map(separated_list1(newline, separated_pair(u64, "-", u64)), merge_ranges),
    double_line_ending,
    {}
))]
struct Input {
    ranges: Vec<(u64, u64)>,
    ingriedients: Vec<u64>,
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable_by_key(|&(start, _)| start);
    ranges.into_iter().fold(Vec::new(), |mut list, (start, end)| {
        if let Some((_, prev_end)) = list.last_mut()
            && start <= *prev_end
        {
            *prev_end = end.max(*prev_end)
        } else {
            list.push((start, end));
        }
        list
    })
}

impl Input {
    fn is_fresh(&self, ingredient: u64) -> bool {
        self.ranges
            .binary_search_by(|&(start, end)| match ingredient {
                _ if start > ingredient => Greater,
                _ if end < ingredient => Less,
                _ => Equal,
            })
            .is_ok()
    }
}

fn calculate_part1(input: &Input) -> usize {
    input.ingriedients.iter().filter(|&ing| input.is_fresh(*ing)).count()
}

fn calculate_part2(input: &Input) -> u64 {
    input.ranges.iter().map(|&(start, end)| end - start + 1).sum()
}

day_main!(Input);

day_test!( 5, example => 3, 14 );
day_test!( 5 => 640, 365804144481581 );
