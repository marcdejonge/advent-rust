#![feature(test)]

use advent_lib::grid::Grid;
use advent_lib::parsing::single_match;
use advent_lib::*;
use bit_set::BitSet;
use nom_parse_macros::parse_from;
use rayon::prelude::*;

#[derive(Clone)]
#[parse_from(map(single_match(AsChar::is_dec_digit), |b| b - b'0' + 1))]
struct Height(u8);

#[inline]
fn for_all_lines<ID, CF, C, LC>(
    grid: &Grid<Height>,
    gen_result_holder: ID,
    combine_function: CF,
    line_context: &LC,
    each_cell: fn(&mut C, &mut LC, &u8, usize),
) -> C
where
    ID: Fn() -> C + Sync + Send,
    CF: Fn(C, C) -> C + Sync + Send,
    C: Clone + Sync + Send,
    LC: Clone + Sync,
{
    [
        Grid::north_lines,
        Grid::east_lines,
        Grid::south_lines,
        Grid::west_lines,
    ]
    .par_iter()
    .map(|line_function| {
        let mut ctx = gen_result_holder();
        for line in line_function(grid) {
            let mut lc = line_context.clone();
            for (loc, value) in line {
                let index = (loc.x() + loc.y() * grid.width()) as usize;
                each_cell(&mut ctx, &mut lc, &value.0, index);
            }
        }
        ctx
    })
    .reduce_with(combine_function)
    .unwrap()
}

fn calculate_part1(tree_heights: &Grid<Height>) -> usize {
    for_all_lines(
        tree_heights,
        BitSet::new,
        |mut curr, next| {
            curr.union_with(&next);
            curr
        },
        &0u8,
        |reachable, max, &height, ix| {
            // for each cell
            if height > *max {
                *max = height;
                reachable.insert(ix);
            }
        },
    )
    .len()
}

fn calculate_part2(tree_heights: &Grid<Height>) -> usize {
    for_all_lines(
        tree_heights,
        || vec![1usize; tree_heights.len()],
        |mut curr, next| {
            for ix in 0..curr.len() {
                curr[ix] *= next[ix];
            }
            curr
        },
        &[0usize; 11],
        |scores, last_seen, &height, ix| {
            // for each cell
            scores[ix] = last_seen[0] - last_seen[height as usize];
            for blocked_height in 0..=height {
                last_seen[blocked_height as usize] = last_seen[0]
            }
            last_seen[0] += 1;
        },
    )
    .iter()
    .max()
    .cloned()
    .unwrap()
}

day_main!(Grid<Height>);
day_test!( 8, example => 21, 8 );
day_test!( 8 => 1700, 470596 );
