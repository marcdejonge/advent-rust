#![feature(test)]

use advent_lib::{
    geometry::point2,
    grid::Grid,
    parsing::{double_line_ending, parsable_pair},
    *,
};
use fxhash::FxHashSet;
use nom_parse_macros::parse_from;

type Point = advent_lib::geometry::Point<2, i32>;

#[parse_from(parsable_pair(double_line_ending))]
struct Input {
    points: Vec<Point>,
    instructions: Vec<Instruction>,
}

#[parse_from]
enum Instruction {
    #[format(preceded("fold along x=", {}))]
    X(i32),
    #[format(preceded("fold along y=", {}))]
    Y(i32),
}

impl Instruction {
    fn apply_to(&self, point: Point) -> Point {
        match self {
            Instruction::X(amount) if point.x() < *amount => point,
            Instruction::X(amount) => point2(amount * 2 - point.x(), point.y()),
            Instruction::Y(amount) if point.y() < *amount => point,
            Instruction::Y(amount) => point2(point.x(), amount * 2 - point.y()),
        }
    }

    fn apply_to_all(&self, points: impl IntoIterator<Item = Point>) -> FxHashSet<Point> {
        points.into_iter().map(|p| self.apply_to(p)).collect()
    }
}

fn calculate_part1(input: &Input) -> usize {
    let instruction = input.instructions.first().unwrap();
    instruction.apply_to_all(input.points.iter().copied()).len()
}

fn calculate_part2(input: &Input) -> String {
    let mut points: FxHashSet<_> = input.points.iter().copied().collect();
    for instruction in &input.instructions {
        points = instruction.apply_to_all(points);
    }

    let width = points.iter().map(|p| p.x()).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y()).max().unwrap() + 1;
    Grid::new_default(' ', width, height).draw_with_overlay(points.iter(), '█')
}

day_main!(Input);

day_test!( 13, example => 17, "Grid(5x5)
┌─────┐
│█████│
│█   █│
│█   █│
│█   █│
│█████│
└─────┘
" );
day_test!( 13 => 621, "Grid(39x6)
┌───────────────────────────────────────┐
│█  █ █  █ █  █   ██  ██   ██    ██ ████│
│█  █ █ █  █  █    █ █  █ █  █    █    █│
│████ ██   █  █    █ █    █  █    █   █ │
│█  █ █ █  █  █    █ █ ██ ████    █  █  │
│█  █ █ █  █  █ █  █ █  █ █  █ █  █ █   │
│█  █ █  █  ██   ██   ███ █  █  ██  ████│
└───────────────────────────────────────┘
" );
