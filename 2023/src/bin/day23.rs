#![feature(test)]

use fxhash::{FxHashMap, FxHashSet};
use petgraph::algo::all_simple_paths;
use petgraph::prelude::*;

use advent_lib::day::*;
use advent_lib::direction::Direction::*;
use advent_lib::direction::{Direction, ALL_DIRECTIONS};
use advent_lib::geometry::Point;
use advent_lib::grid::Grid;
use advent_lib::iter_utils::ZipWithNextTrait;
use advent_macros::FromRepr;

use crate::Cell::*;

struct Day {
    grid: Grid<Cell>,
    start: Point<2, i32>,
    end: Point<2, i32>,
}

impl Day {
    fn get_reachability_graph(
        &self,
        can_move: impl Fn(Point<2, i32>, Point<2, i32>, Direction) -> bool,
    ) -> (NodeIndex, NodeIndex, DiGraph<Point<2, i32>, usize>) {
        let mut g = DiGraph::<Point<2, i32>, usize>::new();
        let mut node_map = FxHashMap::<Point<2, i32>, NodeIndex>::default();
        let mut searched = FxHashSet::<Point<2, i32>>::default();
        let mut stack = Vec::new();
        stack.push(self.start);

        while let Some(curr_pos) = stack.pop() {
            let curr_index = *node_map.entry(curr_pos).or_insert_with(|| g.add_node(curr_pos));
            node_map.insert(curr_pos, curr_index);
            searched.insert(curr_pos);

            for dir in ALL_DIRECTIONS {
                if let Some((next_pos, dist)) = self.find_next_split(curr_pos, dir, &can_move) {
                    let next_index =
                        *node_map.entry(next_pos).or_insert_with(|| g.add_node(next_pos));
                    g.update_edge(curr_index, next_index, dist);

                    if !searched.contains(&next_pos) {
                        stack.push(next_pos);
                    }
                }
            }
        }

        (node_map[&self.start], node_map[&self.end], g)
    }

    fn find_next_split(
        &self,
        from: Point<2, i32>,
        direction: Direction,
        can_move: impl Fn(Point<2, i32>, Point<2, i32>, Direction) -> bool,
    ) -> Option<(Point<2, i32>, usize)> {
        let mut next_pos = from + direction.as_vec();
        if !can_move(from, next_pos, direction) {
            return None;
        }

        let mut next_dir = direction;
        let mut steps = 0;

        loop {
            steps += 1;
            let mut options = [next_dir.turn_left(), next_dir, next_dir.turn_right()]
                .into_iter()
                .filter_map(|dir| {
                    let next_next_pos = next_pos + dir.as_vec();
                    if can_move(next_pos, next_next_pos, dir) {
                        Some((next_next_pos, dir))
                    } else {
                        None
                    }
                });

            if let Some(option) = options.next() {
                if options.next().is_some() {
                    return Some((next_pos, steps)); // Multiple options, this is a cross-road to return
                } else {
                    (next_pos, next_dir) = option;
                }
            } else if next_pos == self.end {
                return Some((next_pos, steps)); // This is the end node, se we should return this
            } else {
                return None; // Dead end, don't return that as an options
            }
        }
    }
}

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, PartialEq)]
enum Cell {
    Ground = b'.',
    Wall = b'#',
    SlideNorth = b'^',
    SlideEast = b'>',
    SlideSouth = b'v',
    SlideWest = b'<',
}

impl Cell {
    fn allow_movement(&self, direction: Direction) -> bool {
        match self {
            Ground => true,
            Wall => false,
            SlideNorth => direction == North,
            SlideEast => direction == East,
            SlideSouth => direction == South,
            SlideWest => direction == West,
        }
    }
}

fn determine_weight<NW>(graph: &DiGraph<NW, usize>, path: Vec<NodeIndex>) -> usize {
    path.into_iter()
        .zip_with_next()
        .map(|(from, to)| graph.edges(from).find(|x| x.target() == to).unwrap().weight())
        .sum()
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let grid = Grid::from(lines);
        let start = grid
            .east_line(0)
            .find(|&(_, c)| c == &Ground)
            .expect("Could not find starting location")
            .0;
        let end = grid
            .east_line(grid.height() - 1)
            .find(|&(_, c)| c == &Ground)
            .expect("Could not find ending location")
            .0;

        Day { grid, start, end }
    }

    fn calculate_part1(&self) -> Self::Output {
        let (start, end, graph) = self.get_reachability_graph(|from, to, dir| {
            if let Some(from_cell) = self.grid.get(from) {
                if from_cell.allow_movement(dir) {
                    if let Some(to_cell) = self.grid.get(to) {
                        return to_cell != &Wall;
                    }
                }
            }
            false
        });

        all_simple_paths(&graph, start, end, 0, None)
            .map(|path| determine_weight(&graph, path))
            .max()
            .unwrap()
    }

    fn calculate_part2(&self) -> Self::Output {
        let (start, end, graph) = self.get_reachability_graph(|_, to, _| {
            if let Some(cell) = self.grid.get(to) {
                cell != &Wall
            } else {
                false
            }
        });

        all_simple_paths(&graph, start, end, 0, None)
            .map(|path| determine_weight(&graph, path))
            .max()
            .unwrap()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 23, example => 94, 154 );
    day_test!( 23 => 2394 ); // Second part is 6554, but is way too slow in testing
}
