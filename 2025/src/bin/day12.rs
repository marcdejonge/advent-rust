#![feature(test)]

use advent_lib::{geometry::Vector, grid::*, iter_utils::CountIf, parsing::*, *};
use nom_parse_macros::parse_from;

#[parse_from(separated_pair(separated_list1(double_line_ending, {}), double_line_ending, {}))]
struct Input {
    shapes: Vec<IndexedShape>,
    problems: Vec<Problem>,
}

#[parse_from(preceded((u32, ":\n"), {}))]
struct IndexedShape {
    shape: Grid<FillField>,
}

#[parse_from(separated_pair({}, ": ", separated_list1(space1, {})))]
struct Problem {
    size: Vector<2, u32>,
    presents: Vec<u32>,
}

fn calculate_part1(input: &Input) -> usize {
    let shape_counters: Vec<_> = input
        .shapes
        .iter()
        .map(|shape| shape.shape.count_if(|(_, f)| f == &&FillField::Filled) as u32)
        .collect();
    input
        .problems
        .iter()
        .filter(|problem| {
            let box_size = problem.size.x() * problem.size.y();
            // If the size is less than the number of blocks the presents take, we are sure it can't fit
            let presents_min_size: u32 =
                problem.presents.iter().zip(shape_counters.iter()).map(|(a, b)| a * b).sum();
            // If we put each present in a 9 block grid, we are guaranteed that it'll work
            let presents_max_size: u32 = 9 * problem.presents.iter().sum::<u32>();

            // Estimate the size, we should be able to use at least half of the space between the shapes
            box_size > (presents_min_size + presents_max_size) / 2
        })
        .count()
}

day_main_half!(Input);

day_test!( 12, example => 2 );
day_test!( 12 => 510 );
