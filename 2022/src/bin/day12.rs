#![feature(test)]

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::search::{a_star_search, SearchGraph, SearchGraphWithGoal};

struct Day {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let nodes: Vec<_> = lines
            .enumerate()
            .flat_map(|(line_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(char_index, char)| Node::new(char, char_index, line_index))
                    .collect::<Vec<_>>()
            })
            .collect();
        let width = nodes.iter().map(|n| n.x).max().unwrap() + 1;
        let height = nodes.iter().map(|n| n.y).max().unwrap() + 1;
        Day { nodes, width, height }
    }

    fn calculate_part1(&self) -> Self::Output {
        struct GraphPart1<'a> {
            graph: &'a Day,
        }

        impl<'a> SearchGraph for GraphPart1<'a> {
            type Node = &'a Node;
            type Score = u32;

            fn neighbours(&self, node: &'a Node) -> Vec<(&'a Node, u32)> {
                let mut neighbours = self.graph.neighbours(node);
                neighbours.retain(|(next, _)| next.height - node.height <= 1);
                neighbours
            }
        }

        impl<'a> SearchGraphWithGoal for GraphPart1<'a> {
            fn is_goal(&self, node: &Node) -> bool { node.c == 'E' }
        }

        let start_node = self.nodes.iter().find(|n| n.c == 'S').expect("Find the starting node");
        a_star_search(&GraphPart1 { graph: self }, start_node)
            .expect("Expected a path from S to E")
            .len()
            - 1
    }

    fn calculate_part2(&self) -> Self::Output {
        struct GraphPart2<'a> {
            graph: &'a Day,
        }

        impl<'a> SearchGraph for GraphPart2<'a> {
            type Node = &'a Node;
            type Score = u32;

            fn neighbours(&self, node: &'a Node) -> Vec<(&'a Node, u32)> {
                let mut neighbours = self.graph.neighbours(node);
                neighbours.retain(|(next, _)| next.height - node.height >= -1);
                neighbours
            }
        }

        impl<'a> SearchGraphWithGoal for GraphPart2<'a> {
            fn is_goal(&self, node: &Node) -> bool { node.height == 0 }
        }

        let start_node = self.nodes.iter().find(|n| n.c == 'E').expect("Find the starting node");
        a_star_search(&GraphPart2 { graph: self }, start_node)
            .expect("Expected a path from E to 0")
            .len()
            - 1
    }
}

impl Day {
    fn neighbours<'a>(&'a self, node: &'a Node) -> Vec<(&'a Node, u32)> {
        let mut neighbours = Vec::with_capacity(4);
        if node.x > 0 {
            neighbours.push((
                self.nodes.get(node.y * self.width + (node.x - 1)).unwrap(),
                1,
            ));
        }
        if node.x < (self.width - 1) {
            neighbours.push((
                self.nodes.get(node.y * self.width + (node.x + 1)).unwrap(),
                1,
            ));
        }
        if node.y > 0 {
            neighbours.push((
                self.nodes.get((node.y - 1) * self.width + node.x).unwrap(),
                1,
            ));
        }
        if node.y < (self.height - 1) {
            neighbours.push((
                self.nodes.get((node.y + 1) * self.width + node.x).unwrap(),
                1,
            ));
        }
        neighbours
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    c: char,
    height: i32,
    x: usize,
    y: usize,
}

impl Node {
    fn new(c: char, x: usize, y: usize) -> Node {
        match c {
            'S' => Node { c, x, y, height: 0 },
            'E' => Node { c, x, y, height: 26 },
            'a'..='z' => Node { c, x, y, height: c as i32 - 'a' as i32 },
            _ => panic!("Unsupported character"),
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 12, example => 31, 29 );
    day_test!( 12 => 383, 377 );
}
