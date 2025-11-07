#![feature(test)]

use advent_lib::{parsing::range_inclusive, *};
use fxhash::{FxHashMap, FxHashSet};
use nom_parse_macros::parse_from;
use std::{cmp::max, ops::RangeInclusive};

#[parse_from(preceded("target area: ", (preceded("x=", range_inclusive(i32, "..")), preceded(", y=", range_inclusive(i32, "..")))))]
struct Input {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

fn calculate_part1(input: &Input) -> i32 {
    let dy = -input.y_range.start() - 1;
    (dy * (dy + 1)) / 2
}

fn calculate_part2(input: &Input) -> usize {
    let mut vx_start_times = FxHashMap::<i32, RangeInclusive<i32>>::default();

    for vx_start in 1..=*input.x_range.end() {
        let mut vx = vx_start;
        let mut x = 0;
        let mut min_time = None;
        for time in 1.. {
            x += vx;
            vx = max(0, vx - 1);
            if x < *input.x_range.start() {
                if vx == 0 {
                    break;
                } else {
                    continue;
                }
            } else if x > *input.x_range.end() {
                if let Some(min_time) = min_time {
                    vx_start_times.insert(vx_start, min_time..=(time - 1));
                }
                break;
            } else if min_time.is_none() {
                min_time = Some(time);
            } else if vx == 0 {
                if let Some(min_time) = min_time {
                    vx_start_times.insert(vx_start, min_time..=i32::MAX);
                }
                break;
            }
        }
    }

    let mut speeds = FxHashSet::<(i32, i32)>::default();
    for vy_start in *input.y_range.start()..(-*input.y_range.start()) {
        // This is time spend in the air before crossing crossing the 0 line again
        let (mut y, mut vy, extra_time) = if vy_start > 0 {
            (0, -vy_start - 1, vy_start * 2 + 1)
        } else {
            (0, vy_start, 0)
        };

        for time in (extra_time + 1).. {
            y += vy;
            vy -= 1;
            if y > *input.y_range.end() {
                continue;
            } else if y < *input.y_range.start() {
                break; // We passed the area, no use looking further
            } else {
                // Found a time that works, now find the related vx_start values
                vx_start_times
                    .iter()
                    .filter(|(_, range)| range.contains(&time))
                    .for_each(|(vx_start, _)| {
                        speeds.insert((*vx_start, vy_start));
                    });
            }
        }
    }

    speeds.len()
}

day_main!(Input);

day_test!( 17, example => 45, 112 );
day_test!( 17 => 4005, 2953 );
