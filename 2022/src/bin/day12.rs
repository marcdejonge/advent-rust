#![feature(test)]

use advent_lib::direction::Direction;
use advent_lib::grid::{Grid, Location};
use advent_lib::parsing::single_match;
use advent_lib::search::{a_star_search, SearchGraph, SearchGraphWithGoal};
use advent_lib::*;
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
    let start_node = grid.find(|height| height.c == b'S').expect("Find the starting node");
    let end_node = grid.find(|height| height.c == b'E').expect("Find the ending node");
    let graph = grid.search_graph(
        end_node,
        |_, current, next| {
            if next.height - current.height <= 1 {
                Some(1)
            } else {
                None
            }
        },
        |_| 0,
    );
    a_star_search(&graph, start_node).expect("Expected a path from S to E").len() - 1
}

fn calculate_part2(grid: &Grid<Node>) -> usize {
    struct GraphPart2<'a> {
        grid: &'a Grid<Node>,
    }

    impl SearchGraph for GraphPart2<'_> {
        type Node = Location;
        type Score = u32;

        fn neighbours(&self, location: Location) -> impl Iterator<Item = (Location, u32)> {
            let current_height = self.grid.get(location).unwrap().height;
            Direction::ALL.into_iter().filter_map(move |dir| {
                let next = location + dir;
                let next_height = self.grid.get(next)?.height;
                if next_height - current_height >= -1 {
                    Some((next, 1))
                } else {
                    None
                }
            })
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

day_main!(Grid<Node>);
day_test!( 12, example => 31, 29 );
day_test!( 12 => 383, 377 );
