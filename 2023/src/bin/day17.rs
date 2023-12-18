/*
--- Day 17: Clumsy Crucible ---

The lava starts flowing rapidly once the Lava Production Facility is operational. As you leave, the
reindeer offers you a parachute, allowing you to quickly reach Gear Island.

As you descend, your bird's-eye view of Gear Island reveals why you had trouble finding anyone on
your way up: half of Gear Island is empty, but the half below you is a giant factory city!

You land near the gradually-filling pool of lava at the base of your new lavafall. Lavaducts will
eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading
it into large crucibles on wheels.

The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult
to steer at high speeds, and so it can be hard to go in a straight line for very long.

To get Desert Island the machine parts it needs as soon as possible, you'll need to find the best
way to get the crucible from the lava pool to the machine parts factory. To do this, you need to
minimize heat loss while choosing a route that doesn't require the crucible to go in a straight line
for too long.

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient
temperature, and hundreds of other parameters to calculate exactly how much heat loss can be
expected for a crucible entering any particular city block.

Each city block is marked by a single digit that represents the amount of heat loss if the crucible
enters that block. The starting point, the lava pool, is the top-left city block; the destination,
the machine parts factory, is the bottom-right city block. (Because you already start in the
top-left block, you don't incur that block's heat loss unless you leave that block and then return
to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it
can move at most three blocks in a single direction before it must turn 90 degrees left or right.
The crucible also can't reverse direction; after entering each city block, it may only turn left,
continue straight, or turn right.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than
three consecutive blocks in the same direction, what is the least heat loss it can incur?

--- Part Two ---

The crucibles of lava simply aren't large enough to provide an adequate supply of lava to the
machine parts factory. Instead, the Elves are going to upgrade to ultra crucibles.

Ultra crucibles are even more difficult to steer than normal crucibles. Not only do they have
trouble going in a straight line, but they also have trouble turning!

Once an ultra crucible starts moving in a direction, it needs to move a minimum of four blocks in
that direction before it can turn (or even before it can stop at the end). However, it will
eventually start to get wobbly: an ultra crucible can move a maximum of ten consecutive blocks
without turning.

Directing the ultra crucible from the lava pool to the machine parts factory, what is the least heat
loss it can incur?
*/
#![feature(test)]

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::search::{a_star_search, SearchGraph, SearchGraphWithGoal};

struct Day {
    grid: Grid<u8>,
}

struct Target<'a> {
    grid: &'a Grid<u8>,
    min_steps: i32,
    max_steps: i32,
}

impl<'a> Target<'a> {
    fn goal(&self) -> Point<2, i32> { point2(self.grid.width() - 1, self.grid.height() - 1) }

    fn calc_path_cost(&self, path: &[(Point<2, i32>, Direction, i32)]) -> usize {
        path.iter()
            .filter_map(
                |(loc, _, _)| if loc.x() == 0 && loc.y() == 0 { None } else { self.grid.get(*loc) },
            )
            .map(|b| *b as usize)
            .sum()
    }
}

impl<'a> SearchGraph for Target<'a> {
    // Location plus number of straight steps we took
    type Node = (Point<2, i32>, Direction, i32);
    type Score = i32;

    fn neighbours(&self, node: Self::Node) -> Vec<(Self::Node, Self::Score)> {
        let (curr_loc, curr_dir, straight_steps) = node;
        let mut neighbours = Vec::with_capacity(3);

        if straight_steps >= self.min_steps || straight_steps == 0 {
            let left = curr_dir.turn_left();
            let left_loc = curr_loc + left.as_vec();
            let left_cell = self.grid.get(left_loc);
            if let Some(cell) = left_cell {
                neighbours.push(((left_loc, left, 1), *cell as i32));
            }

            let right = curr_dir.turn_right();
            let right_loc = curr_loc + right.as_vec();
            let right_cell = self.grid.get(right_loc);
            if let Some(cell) = right_cell {
                neighbours.push(((right_loc, right, 1), *cell as i32));
            }
        }

        if straight_steps < self.max_steps {
            let straight_loc = curr_loc + curr_dir.as_vec();
            let straight_cell = self.grid.get(straight_loc);
            if let Some(cell) = straight_cell {
                neighbours.push(((straight_loc, curr_dir, straight_steps + 1), *cell as i32));
            }
        }

        neighbours
    }
}

impl<'a> SearchGraphWithGoal for Target<'a> {
    fn is_goal(&self, node: Self::Node) -> bool {
        self.goal() == node.0 && node.2 >= self.min_steps
    }

    fn heuristic(&self, node: Self::Node) -> Self::Score { (self.goal() - node.0).euler() }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut grid = Grid::from(lines);
        grid.entries_mut().for_each(|(_, cell)| *cell -= b'0');
        Day { grid }
    }

    fn calculate_part1(&self) -> Self::Output {
        let target = &Target { grid: &self.grid, min_steps: 1, max_steps: 3 };
        let path = a_star_search(target, (point2(0, 0), East, 0)).unwrap();
        target.calc_path_cost(&path)
    }

    fn calculate_part2(&self) -> Self::Output {
        let target = &Target { grid: &self.grid, min_steps: 4, max_steps: 10 };
        let path = a_star_search(target, (point2(0, 0), East, 0)).unwrap();
        target.calc_path_cost(&path)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 17, example => 102, 94 );
    day_test!( 17 => 866, 1010 );
}
