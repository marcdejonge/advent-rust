#![feature(test)]

use advent_lib::{parsing::*, *};
use bit_set::BitSet;
use itertools::{Itertools, iterate};
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use smallvec::SmallVec;

#[parse_from((
    in_brackets(map(is_a(".#"), |input: I| {
        input.as_bytes().iter().enumerate().fold(0, |accum, (ix, b)| {
            if *b == b'#' {
                accum | (1 << ix)
            } else {
                accum
            }
        })
    })),
    delimited(space0, separated_list1(space1, in_parens(map(separated_list1(",", u32), |nrs| {
        nrs.iter().fold(0, |accum, nr| {
            accum | (1 << nr)
        })
    }))), space0),
    in_braces(separated_list1(",", u32)),
))]
#[derive(Debug)]
struct Line {
    target: usize,
    switches: Vec<u32>,
    joltages: Vec<u32>,
}

fn calculate_part1(lines: &[Line]) -> usize {
    lines
        .par_iter()
        .filter_map(|line| {
            let mut initial = BitSet::new();
            initial.insert(0);
            iterate(initial, |last| {
                last.iter()
                    .flat_map(|nr| line.switches.iter().map(move |s| nr ^ (*s as usize)))
                    .collect()
            })
            .find_position(|nrs| nrs.contains(line.target))
            .map(|(depth, _)| depth)
        })
        .sum()
}

fn apply_switch(switch: u32, current: &mut [u32], count: u32) {
    if count > 0 {
        current.iter_mut().enumerate().for_each(|(ix, v)| {
            if switch & (1 << ix) != 0 {
                *v += count
            }
        });
    }
}

fn unapply_switch(switch: u32, current: &mut [u32], count: u32) {
    if count > 0 {
        current.iter_mut().enumerate().for_each(|(ix, v)| {
            if switch & (1 << ix) != 0 {
                *v -= count
            }
        });
    }
}

fn find_max_count(target: &[u32], current: &[u32], switch: u32) -> u32 {
    (0..target.len())
        .filter(|ix| switch & (1 << ix) != 0)
        .map(|ix| target[ix] - current[ix])
        .min()
        .unwrap()
}

fn part2_find(
    switches: &mut [u32],
    target: &[u32],
    current: &mut [u32],
    press_count: usize,
    #[cfg(feature = "debug_print")] indent: &str,
) -> Option<usize> {
    assert_eq!(target.len(), current.len());

    if target == current {
        #[cfg(feature = "debug_print")]
        eprintln!(
            "{}Found target {:?} after {} presses",
            indent, target, press_count
        );
        return Some(press_count);
    } else if current.iter().zip(target.iter()).any(|(curr, tar)| curr > tar) {
        #[cfg(feature = "debug_print")]
        eprintln!(
            "{}Overshot the target {:?} with {:?}",
            indent, target, current
        );
        return None;
    } else {
        #[cfg(feature = "debug_print")]
        eprintln!(
            "{}Finding for part2({:?}, {:?}, {:?}, {})",
            indent, switches, target, current, press_count
        );
    }

    let (joltage_ix, selected_switch_indices) = (0..target.len())
        .filter(|&ix| current[ix] < target[ix])
        .map(|ix| {
            (
                ix,
                switches
                    .iter()
                    .enumerate()
                    .filter(|(_, switch)| **switch & (1 << ix) != 0)
                    .map(|(s_ix, _)| s_ix)
                    .collect(),
            )
        })
        .min_by_key(|(_, options): &(usize, SmallVec<[usize; 16]>)| options.len())?;

    match *selected_switch_indices.as_slice() {
        [] => {
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}There are incomplete parts, but no switch to change that",
                indent
            );
            None
        }
        [selected_ix] => {
            let selected_switch = switches[selected_ix];
            let count = target[joltage_ix] - current[joltage_ix];
            apply_switch(selected_switch, current, count);
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Found single switch to apply {} {} times to get {:?}",
                indent, selected_switch, count, current
            );
            switches[selected_ix] = 0; // Effectively disabling that switch
            let result = part2_find(
                switches,
                target,
                current,
                press_count + count as usize,
                #[cfg(feature = "debug_print")]
                &(indent.to_owned() + "  "),
            );
            switches[selected_ix] = selected_switch; // And undo-ing the last step
            unapply_switch(selected_switch, current, count);
            result
        }
        [first_ix, second_ix] => {
            let first_switch = switches[first_ix];
            let second_switch = switches[second_ix];
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Try to apply 2 switches {} total {} times to get {:?}",
                indent, switches, count, current
            );

            let count = target[joltage_ix] - current[joltage_ix];
            let first_max_count = find_max_count(target, current, first_switch);
            let second_max_count = find_max_count(target, current, second_switch);
            if first_max_count + second_max_count < count {
                return None;
            }

            apply_switch(first_switch, current, first_max_count);
            apply_switch(second_switch, current, count - first_max_count);
            switches[first_ix] = 0;
            switches[second_ix] = 0;
            let apply_count = second_max_count - (count - first_max_count);

            let result = (0..=apply_count)
                .filter_map(|ix| {
                    if ix > 0 {
                        unapply_switch(first_switch, current, 1);
                        apply_switch(second_switch, current, 1);
                    }

                    part2_find(
                        switches,
                        target,
                        current,
                        press_count + count as usize,
                        #[cfg(feature = "debug_print")]
                        &(indent.to_owned() + "  "),
                    )
                })
                .min();

            switches[first_ix] = first_switch;
            switches[second_ix] = second_switch;
            unapply_switch(first_switch, current, first_max_count - apply_count);
            unapply_switch(second_switch, current, second_max_count);

            result
        }
        _ => {
            // There are multiple switches we can use, try the most complicated one first
            let (_, selected_switch_ix, selected_switch) = selected_switch_indices
                .iter()
                .map(|&ix| {
                    let switch = switches[ix];
                    (switch.count_ones(), ix, switch)
                })
                .max()
                .unwrap();
            let max_count = find_max_count(target, current, selected_switch);
            switches[selected_switch_ix] = 0;
            apply_switch(selected_switch, current, max_count + 1);

            let result = (0..=max_count)
                .rev()
                .filter_map(|count| {
                    unapply_switch(selected_switch, current, 1);
                    #[cfg(feature = "debug_print")]
                    eprintln!(
                        "{}Trying to apply switch {} {} times to get {:?}",
                        indent, selected_switch, count, current
                    );
                    part2_find(
                        switches,
                        target,
                        current,
                        press_count + count as usize,
                        #[cfg(feature = "debug_print")]
                        &(indent.to_owned() + "  "),
                    )
                })
                .min();
            switches[selected_switch_ix] = selected_switch;
            result
        }
    }
}

fn calculate_part2(lines: &[Line]) -> usize {
    #[allow(unused_variables)]
    lines
        .par_iter()
        .enumerate()
        .map(|(ix, line)| {
            #[cfg(feature = "debug_print")]
            eprintln!("   --- Started on line {} ---", ix + 1);
            let mut switches = line.switches.clone();
            let result = part2_find(
                &mut switches,
                &line.joltages,
                &mut vec![0; line.joltages.len()],
                0,
                #[cfg(feature = "debug_print")]
                "",
            )
            .unwrap();
            #[cfg(feature = "debug_print")]
            eprintln!("   --- Finished on line {} ---", ix + 1);
            result
        })
        .sum()
}

day_main!(Vec<Line>);

day_test!( 10, example => 7, 33 );
day_test!( 10 => 425 ); // 15883 for the second part, but it's way too slow for testing right now
