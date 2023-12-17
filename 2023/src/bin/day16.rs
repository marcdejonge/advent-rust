/*
--- Day 16: The Floor Will Be Lava ---

With the beam of light completely focused somewhere, the reindeer leads you deeper still into the
Lava Production Facility. At some point, you realize that the steel facility walls have been
replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure
this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern
up ahead. There, you discover that the beam of light you so carefully focused is emerging from the
cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite
side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing
empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the
grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input).

The beam enters in the top-left corner from the left and heading to the right. Then, its behavior
depends on what it encounters as it moves:

- If the beam encounters empty space (.), it continues in the same direction.
- If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle
of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue
upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would
continue downward from the mirror's column.
- If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter
as if the splitter were empty space. For instance, a rightward-moving beam that encounters a -
splitter would continue in the same direction.
- If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going
in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-
moving beam that encounters a | splitter would split into two beams: one that continues upward from
the splitter's column and one that continues downward from the splitter's column.

Beams do not interact with other beams; a tile can have many beams passing through it at the same
time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or
split in it.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start
by analyzing the current situation. With the beam starting in the top-left heading right, how many
tiles end up being energized?

--- Part Two ---

As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a
nearby control panel. There, a collection of buttons lets you align the contraption so that the beam
enters from any edge tile and heading away from that edge. (You can choose either of two directions
for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner,
it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row
(heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost
column (heading left). To produce lava, you need to find the configuration that energizes as many
tiles as possible.

Find the initial beam configuration that energizes the largest number of tiles; how many tiles are
energized in that configuration?
*/
#![feature(test)]
#![feature(iter_collect_into)]

use fxhash::FxHashSet;
use rayon::prelude::*;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::direction::Direction::*;
use advent_lib::geometry::{point2, Point};
use advent_lib::grid::Grid;
use advent_lib::search::depth_first_search;
use advent_macros::FromRepr;

struct Day {
    grid: Grid<Mirror>,
}

impl Day {
    fn neighbours(
        &self,
        curr_loc: Point<2, i32>,
        curr_dir: Direction,
    ) -> Vec<(Point<2, i32>, Direction)> {
        let mirror = self.grid[curr_loc];

        let mut neighbours = Vec::with_capacity(2);
        let mut add = |next_dir: Direction| {
            let next_loc = curr_loc + next_dir.as_vec();
            if self.grid.is_valid_location(&next_loc) {
                neighbours.push((next_loc, next_dir));
            }
        };

        match (mirror, curr_dir) {
            (Mirror::None, _)
            | (Mirror::SplitHor, East)
            | (Mirror::SplitHor, West)
            | (Mirror::SplitVer, North)
            | (Mirror::SplitVer, South) => add(curr_dir),
            (Mirror::TurnLeft, East) => add(North),
            (Mirror::TurnLeft, North) => add(East),
            (Mirror::TurnLeft, West) => add(South),
            (Mirror::TurnLeft, South) => add(West),
            (Mirror::TurnRight, East) => add(South),
            (Mirror::TurnRight, South) => add(East),
            (Mirror::TurnRight, West) => add(North),
            (Mirror::TurnRight, North) => add(West),
            (Mirror::SplitHor, _) => {
                add(East);
                add(West);
            }
            (Mirror::SplitVer, _) => {
                add(North);
                add(South);
            }
        }

        neighbours
    }

    fn visit_grid(&self, start_loc: Point<2, i32>, start_dir: Direction) -> usize {
        let mut energized_grid = Grid::new_empty(self.grid.width(), self.grid.height());
        let mut visited = FxHashSet::<(Point<2, i32>, Direction)>::default();
        visited.insert((start_loc, start_dir));

        depth_first_search(
            (start_loc, start_dir),
            |(curr_loc, curr_dir)| self.neighbours(curr_loc, curr_dir),
            |state| visited.insert(state),
        );

        visited
            .into_iter()
            .for_each(|(loc, _)| *energized_grid.get_mut(loc).unwrap() = true);

        energized_grid.values().filter(|b| **b).count()
    }
}

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, PartialEq, Hash)]
enum Mirror {
    None = b'.',
    SplitHor = b'-',
    SplitVer = b'|',
    TurnRight = b'\\',
    TurnLeft = b'/',
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { grid: Grid::from(lines) }
    }

    fn calculate_part1(&self) -> Self::Output { self.visit_grid(point2(0, 0), East) }

    fn calculate_part2(&self) -> Self::Output {
        [
            self.grid
                .x_range()
                .map(|x| (point2(x, self.grid.height() - 1), North))
                .collect::<Vec<_>>(),
            self.grid.y_range().map(|y| (point2(0, y), East)).collect(),
            self.grid.x_range().map(|x| (point2(x, 0), South)).collect(),
            self.grid.y_range().map(|y| (point2(self.grid.width() - 1, y), West)).collect(),
        ]
        .into_iter()
        .flatten()
        .par_bridge()
        .map(|(start, dir)| self.visit_grid(start, dir))
        .max()
        .unwrap()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 16, example => 46, 51 );
    day_test!( 16 => 7482, 7896 );
}
