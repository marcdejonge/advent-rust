#![feature(test)]

use std::iter::successors;

use advent_lib::{
    builder::with_default,
    direction::Direction::{self, *},
    geometry::point2,
    grid::{Grid, Location},
    math::greatest_common_divisor,
    search::{a_star_search, SearchGraph, SearchGraphWithGoal},
    *,
};
use advent_macros::FromRepr;

#[repr(u8)]
#[derive(FromRepr, Default, Copy, Clone)]
enum InputBlock {
    Blocked = b'#',
    #[default]
    Empty = b'.',
    StormUp = b'^',
    StormDown = b'v',
    StormLeft = b'<',
    StormRight = b'>',
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Storm(u8);

impl Storm {
    const EMPTY: Storm = Storm(0);

    fn set_blow(&mut self, dir: Direction) { self.0 |= 1 << usize::from(dir) }

    fn get_blow(&self, dir: Direction) -> bool { self.0 & (1 << usize::from(dir)) != 0 }
}

impl From<InputBlock> for Storm {
    fn from(value: InputBlock) -> Self {
        match value {
            InputBlock::Blocked => panic!("Unsupported"),
            InputBlock::Empty => Storm::default(),
            InputBlock::StormUp => with_default(|storm: &mut Storm| storm.set_blow(North)),
            InputBlock::StormDown => with_default(|storm: &mut Storm| storm.set_blow(South)),
            InputBlock::StormLeft => with_default(|storm: &mut Storm| storm.set_blow(West)),
            InputBlock::StormRight => with_default(|storm: &mut Storm| storm.set_blow(East)),
        }
    }
}

impl From<Storm> for char {
    fn from(value: Storm) -> Self {
        match value.0 {
            0 => '.',
            1 => '>',
            2 => 'v',
            4 => '<',
            8 => '^',
            _ => char::from_digit(value.0.count_ones(), 10).unwrap(),
        }
    }
}

struct StormTimeline {
    grids: Vec<Grid<Storm>>,
    start: Location,
    end: Location,
}

impl StormTimeline {
    fn step_storm(grid: &Grid<Storm>) -> Grid<Storm> {
        let mut result = Grid::<Storm>::new_empty(grid.width(), grid.height());

        for (loc, storm) in result.entries_mut() {
            for dir in Direction::ALL {
                if grid.get_infinite(loc - dir).get_blow(dir) {
                    storm.set_blow(dir)
                }
            }
        }

        result
    }

    fn new(grid: Grid<InputBlock>) -> StormTimeline {
        let initial_grid = grid
            .sub_grid(1..grid.width() - 1, 1..grid.height() - 1)
            .map(|&b| Storm::from(b));
        let (width, height) = initial_grid.size().into();
        let grids = successors(Some(initial_grid), |grid| Some(Self::step_storm(grid)))
            .take(((width * height) / greatest_common_divisor(width, height)) as usize)
            .collect();
        StormTimeline { grids, start: point2(0, -1), end: point2(width - 1, height) }
    }

    fn get(&self, time: usize) -> &Grid<Storm> { self.grids.get(time % self.grids.len()).unwrap() }
    fn graph_to_start<'a>(&'a self) -> StormGraph<'a> {
        StormGraph { timeline: self, goal: self.start }
    }
    fn graph_to_end<'a>(&'a self) -> StormGraph<'a> {
        StormGraph { timeline: self, goal: self.end }
    }
}

struct StormGraph<'a> {
    timeline: &'a StormTimeline,
    goal: Location,
}

impl SearchGraph for StormGraph<'_> {
    type Node = (Location, usize);

    type Score = i32;

    fn neighbours(
        &self,
        (loc, time): Self::Node,
    ) -> impl Iterator<Item = (Self::Node, Self::Score)> {
        let next_grid = self.timeline.get(time + 1);
        [loc, loc + North, loc + East, loc + South, loc + West]
            .into_iter()
            .filter(move |&loc| {
                next_grid.get(loc) == Some(&Storm::EMPTY)
                    || loc == self.timeline.start
                    || loc == self.timeline.end
            })
            .map(move |loc| ((loc, time + 1), 1))
    }
}

impl SearchGraphWithGoal for StormGraph<'_> {
    fn is_goal(&self, (loc, _): Self::Node) -> bool { loc == self.goal }
    fn heuristic(&self, (loc, _): Self::Node) -> Self::Score { (self.goal - loc).euler() }
}

fn last_time(steps: Option<Vec<(Location, usize)>>) -> usize { steps.unwrap()[0].1 }

fn calculate_part1(tl: &StormTimeline) -> usize {
    last_time(a_star_search(&tl.graph_to_end(), (tl.start, 0)))
}

fn calculate_part2(tl: &StormTimeline) -> usize {
    let mut time = last_time(a_star_search(&tl.graph_to_end(), (tl.start, 0)));
    time = last_time(a_star_search(&tl.graph_to_start(), (tl.end, time)));
    last_time(a_star_search(&tl.graph_to_end(), (tl.start, time)))
}

day_main!( StormTimeline::new => calculate_part1, calculate_part2 );

day_test!( 24, example => 18, 54 ; crate::StormTimeline::new );
day_test!( 24 => 297, 856 ; crate::StormTimeline::new );
