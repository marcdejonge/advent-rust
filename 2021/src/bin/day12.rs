#![feature(test)]

use advent_lib::{key::Key, parsing::peek_char_mapped, *};
use bit_set::BitSet;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;

const START: Key = Key::fixed(b"start");
const END: Key = Key::fixed(b"end");

#[parse_from(map(
    separated_list1(line_ending, separated_pair({}, "-", {})),
    |links: Vec<(Cave, Cave)>| {
        let mut ids = FxHashMap::<Key, usize>::default();
        let mut caves = Vec::<Cave>::new();
        ids.insert(START, 0);
        caves.push(Cave { key: START, is_small: true, paths: Vec::new() });
        ids.insert(END, 1);
        caves.push(Cave { key: END, is_small: true, paths: Vec::new() });

        for (from, to) in links {
            let from_id = *ids
                .entry(from.key)
                .or_insert_with_key(|_| {
                    let next_id = caves.len();
                    caves.push(from.clone());
                    next_id
                });
            let to_id = *ids
                .entry(to.key)
                .or_insert_with_key(|_| {
                    let next_id = caves.len();
                    caves.push(to.clone());
                    next_id
                });
            caves[from_id].paths.push(to_id);
            caves[to_id].paths.push(from_id);
        }

        caves
    }
))]
struct Caves(Vec<Cave>);

#[derive(Clone)]
#[parse_from((peek_char_mapped(char::is_ascii_lowercase), {}))]
struct Cave {
    is_small: bool,
    key: Key,
    #[derived(Default::default())]
    paths: Vec<usize>,
}

impl Caves {
    fn count_all_paths_to_end(&self, allow_duplicate: bool) -> usize {
        self.count_paths_to_end(&mut BitSet::with_capacity(self.0.len()), 0, allow_duplicate)
    }

    fn count_paths_to_end(
        &self,
        visited: &mut BitSet,
        current: usize,
        allow_duplicate: bool,
    ) -> usize {
        let mut count = 0;
        for &next_id in &self.0.get(current).unwrap().paths {
            match next_id {
                0 => {}          // Just continue exploring, we can never go back to the start
                1 => count += 1, // Found a route, count it and continue other paths
                _ => {
                    let next_cave = self.0.get(next_id).unwrap();
                    if next_cave.is_small && visited.contains(next_id) {
                        if allow_duplicate {
                            count += self.count_paths_to_end(visited, next_id, false)
                        }
                    } else {
                        visited.insert(next_id);
                        count += self.count_paths_to_end(visited, next_id, allow_duplicate);
                        visited.remove(next_id);
                    }
                }
            }
        }
        count
    }
}

fn calculate_part1(caves: &Caves) -> usize {
    caves.count_all_paths_to_end(false)
}

fn calculate_part2(caves: &Caves) -> usize {
    caves.count_all_paths_to_end(true)
}

day_main!(Caves);

day_test!( 12, small => 10, 36 );
day_test!( 12, example => 19, 103 );
day_test!( 12, example2 => 226, 3509 );
day_test!( 12 => 4754, 143562 );
