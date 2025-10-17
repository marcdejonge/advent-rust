#![feature(test)]

use advent_lib::{
    builder::with_default,
    direction::Direction::{self, *},
    geometry::point2,
    grid::{Grid, Location},
    iter_utils::CountIf,
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
struct Storm([bool; 4]);

impl Storm {
    const EMPTY: Storm = Storm([false; 4]);

    fn set_blow(&mut self, dir: Direction) { self.0[usize::from(dir)] = true }

    fn get_blow(&self, dir: Direction) -> bool { self.0[usize::from(dir)] }
}

impl From<Direction> for Storm {
    fn from(value: Direction) -> Self { with_default::<Self, _>(|it| it.set_blow(value)) }
}

impl From<InputBlock> for Storm {
    fn from(value: InputBlock) -> Self {
        match value {
            InputBlock::Blocked => panic!("Unsupported"),
            InputBlock::Empty => Storm::default(),
            InputBlock::StormUp => North.into(),
            InputBlock::StormDown => South.into(),
            InputBlock::StormLeft => West.into(),
            InputBlock::StormRight => East.into(),
        }
    }
}

impl From<Storm> for char {
    fn from(value: Storm) -> Self {
        let count = value.0.count_if(|b| *b) as u32;
        if count == 0 {
            '.'
        } else if count == 1 {
            match value.0 {
                [true, false, false, false] => '>',
                [false, true, false, false] => 'v',
                [false, false, true, false] => '<',
                [false, false, false, true] => '^',
                _ => 'X',
            }
        } else {
            char::from_digit(count, 10).unwrap()
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
        let grid = grid
            .sub_grid(1..grid.width() - 1, 1..grid.height() - 1)
            .map(|&b| Storm::from(b));
        let repeat = ((grid.width() * grid.height())
            / greatest_common_divisor(grid.width(), grid.height())) as usize;
        let mut grids = Vec::with_capacity(repeat);
        grids.push(grid.clone());
        for _ in 1..repeat {
            grids.push(Self::step_storm(grids.last().unwrap()));
        }

        StormTimeline { grids, start: point2(0, -1), end: point2(grid.width() - 1, grid.height()) }
    }

    fn get(&self, time: usize) -> &Grid<Storm> { self.grids.get(time % self.grids.len()).unwrap() }
    fn to_graph<'a>(&'a self, goal: Location) -> StormGraph<'a> {
        StormGraph { timeline: self, goal }
    }
    fn graph_to_start<'a>(&'a self) -> StormGraph<'a> { self.to_graph(self.start) }
    fn graph_to_end<'a>(&'a self) -> StormGraph<'a> { self.to_graph(self.end) }
}

struct StormGraph<'a> {
    timeline: &'a StormTimeline,
    goal: Location,
}

impl SearchGraph for StormGraph<'_> {
    type Node = (Location, usize);

    type Score = i32;

    fn neighbours(&self, (loc, time): Self::Node) -> Vec<(Self::Node, Self::Score)> {
        // self.timeline.get(time).draw_with_overlay([&loc], 'E');
        let grid = self.timeline.get(time + 1);
        [loc, loc + North, loc + East, loc + South, loc + West]
            .iter()
            .filter(|&&loc| {
                grid.get(loc) == Some(&Storm::EMPTY)
                    || loc == self.timeline.start
                    || loc == self.timeline.end
            })
            .map(|&loc| ((loc, time + 1), 1))
            .collect::<Vec<_>>()
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
