#![feature(test)]

use advent_lib::day_main;
use advent_lib::direction::ALL_DIRECTIONS;
use advent_lib::grid::{Grid, Location};
use advent_lib::parsing::single_match;
use advent_lib::search::{a_star_search, SearchGraph, SearchGraphWithGoal};
use nom_parse_macros::parse_from;

#[parse_from(map(single_match(AsChar::is_alpha), |value: u8| (value, match value {
    b'S' => 0,
    b'E' => 26,
    b'a'..=b'z' => (value - b'a') as i32,
    _ => panic!("Unsupported character"),
})))]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    c: u8,
    height: i32,
}

impl From<u8> for Node {
    fn from(value: u8) -> Self {
        Node {
            c: value,
            height: match value {
                b'S' => 0,
                b'E' => 26,
                b'a'..=b'z' => (value - b'a') as i32,
                _ => panic!("Unsupported character"),
            },
        }
    }
}

fn calculate_part1(grid: &Grid<Node>) -> usize {
    struct GraphPart1<'a> {
        grid: &'a Grid<Node>,
    }

    impl SearchGraph for GraphPart1<'_> {
        type Node = Location;
        type Score = u32;

        fn neighbours(&self, location: Location) -> Vec<(Location, u32)> {
            let current_height = self.grid.get(location).unwrap().height;
            ALL_DIRECTIONS
                .iter()
                .filter_map(|&dir| {
                    let next = location + dir;
                    let next_height = self.grid.get(next)?.height;
                    if next_height - current_height <= 1 {
                        Some((next, 1))
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    impl SearchGraphWithGoal for GraphPart1<'_> {
        fn is_goal(&self, location: Location) -> bool { self.grid.get(location).unwrap().c == b'E' }
    }

    let start_node = grid.find(|height| height.c == b'S').expect("Find the starting node");
    a_star_search(&GraphPart1 { grid }, start_node)
        .expect("Expected a path from S to E")
        .len()
        - 1
}

fn calculate_part2(grid: &Grid<Node>) -> usize {
    struct GraphPart2<'a> {
        grid: &'a Grid<Node>,
    }

    impl SearchGraph for GraphPart2<'_> {
        type Node = Location;
        type Score = u32;

        fn neighbours(&self, location: Location) -> Vec<(Location, u32)> {
            let current_height = self.grid.get(location).unwrap().height;
            ALL_DIRECTIONS
                .iter()
                .filter_map(|&dir| {
                    let next = location + dir;
                    let next_height = self.grid.get(next)?.height;
                    if next_height - current_height >= -1 {
                        Some((next, 1))
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    impl SearchGraphWithGoal for GraphPart2<'_> {
        fn is_goal(&self, location: Location) -> bool {
            self.grid.get(location).unwrap().height == 0
        }
    }

    let start_node = grid.find(|height| height.c == b'E').expect("Find the starting node");
    a_star_search(&GraphPart2 { grid }, start_node)
        .expect("Expected a path from E to 0")
        .len()
        - 1
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 12, example => 31, 29 );
    day_test!( 12 => 383, 377 );
}
