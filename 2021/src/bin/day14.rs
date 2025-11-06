#![feature(test)]
#![feature(array_windows)]

use advent_lib::{
    builder::with_default,
    iter_utils::IteratorUtils,
    parsing::{char_alpha, double_line_ending, parser_to_string},
    *,
};
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use std::ops::{Add, AddAssign};

#[parse_from(separated_pair(
    parser_to_string(take_while(AsChar::is_alpha)),
    double_line_ending,
    map(
        separated_list1(line_ending, separated_pair((char_alpha(), char_alpha()), " -> ", char_alpha())),
       |vec| vec.into_iter().collect()
    )
))]
struct Input {
    start: String,
    rules: FxHashMap<(char, char), char>,
}

#[derive(Clone, Default)]
struct CharCount(FxHashMap<char, usize>);

impl CharCount {
    fn score(&self) -> usize {
        self.0.values().max().unwrap() - self.0.values().min().unwrap()
    }
}

impl Add<CharCount> for CharCount {
    type Output = CharCount;

    fn add(mut self, rhs: CharCount) -> Self::Output {
        for (char, count) in rhs.0 {
            *self.0.entry(char).or_default() += count;
        }
        self
    }
}

impl AddAssign<char> for CharCount {
    fn add_assign(&mut self, rhs: char) {
        *self.0.entry(rhs).or_default() += 1;
    }
}

struct Memoized {
    results: FxHashMap<((char, char), usize), CharCount>,
}

impl Input {
    fn score_from(&self, steps_left: usize) -> usize {
        let mut memory = Memoized {
            results: Default::default(),
        };
        self.start
            .chars()
            .windowed::<2>()
            .map(|chars| self.count(&mut memory, (chars[0], chars[1]), steps_left))
            .fold(
                with_default(|cc| *cc += self.start.chars().last().unwrap()),
                CharCount::add,
            )
            .score()
    }

    fn count(&self, memory: &mut Memoized, from: (char, char), steps_left: usize) -> CharCount {
        if let Some(result) = memory.results.get(&(from, steps_left)) {
            result.clone()
        } else {
            let result = if steps_left == 0 {
                with_default(|char_count: &mut CharCount| {
                    *char_count += from.0;
                })
            } else {
                let insert = *self.rules.get(&from).unwrap();
                self.count(memory, (from.0, insert), steps_left - 1)
                    + self.count(memory, (insert, from.1), steps_left - 1)
            };
            memory.results.insert((from, steps_left), result.clone());
            result
        }
    }
}

fn calculate_part1(input: &Input) -> usize {
    input.score_from(10)
}

fn calculate_part2(input: &Input) -> usize {
    input.score_from(40)
}

day_main!(Input);

day_test!( 14, example => 1588, 2188189693529 );
day_test!( 14 => 3831, 5725739914282 );
