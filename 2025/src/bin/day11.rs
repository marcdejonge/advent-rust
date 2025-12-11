#![feature(test)]

use advent_lib::{key::Key, parsing::separated_map1, *};
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;

#[parse_from(separated_map1(newline, separated_pair({}, ": ", separated_list1(space1, {}))))]
struct Input {
    links: FxHashMap<Key, Vec<Key>>,
}

struct PathCounter<'a> {
    input: &'a Input,
    mem: FxHashMap<Key, u64>,
}

impl<'a> PathCounter<'a> {
    fn with_goal(input: &'a Input, goal: Key) -> Self {
        let mut mem = FxHashMap::default();
        mem.insert(goal, 1);
        Self { input, mem }
    }

    fn paths_from(&mut self, start: Key) -> u64 {
        if let Some(&result) = self.mem.get(&start) {
            result
        } else if let Some(targets) = self.input.links.get(&start) {
            // To prevent endless loops when there is a cycle, we assume 0 steps here when coming back
            self.mem.insert(start, 0);
            let result = targets.iter().map(|&target| self.paths_from(target)).sum();
            self.mem.insert(start, result);
            result
        } else {
            0
        }
    }
}

fn calculate_part1(input: &Input) -> u64 {
    PathCounter::with_goal(input, Key::fixed(b"out")).paths_from(Key::fixed(b"you"))
}

fn calculate_part2(input: &Input) -> u64 {
    PathCounter::with_goal(input, Key::fixed(b"fft")).paths_from(Key::fixed(b"svr"))
        * PathCounter::with_goal(input, Key::fixed(b"dac")).paths_from(Key::fixed(b"fft"))
        * PathCounter::with_goal(input, Key::fixed(b"out")).paths_from(Key::fixed(b"dac"))
}

day_main!(Input);

day_test!( 11, example1 => 5 );
day_test!( 11, example2 => 8, 2 );
day_test!( 11 => 477, 383307150903216 );
