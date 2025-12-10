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

#[derive(Clone)]
struct Switch(u32);

impl Switch {
    fn does_switch(&self, ix: usize) -> bool { self.0 & (1 << ix) != 0 }

    fn apply(&self, current: &mut [u32], count: u32) {
        if count > 0 {
            current.iter_mut().enumerate().for_each(|(ix, v)| {
                if self.does_switch(ix) {
                    *v += count
                }
            });
        }
    }

    fn unapply(&self, current: &mut [u32], count: u32) {
        if count > 0 {
            current.iter_mut().enumerate().for_each(|(ix, v)| {
                if self.does_switch(ix) {
                    *v -= count
                }
            });
        }
    }
}

fn find_max_count(target: &[u32], current: &[u32], switch: &Switch) -> u32 {
    (0..target.len())
        .filter(|&ix| switch.does_switch(ix))
        .map(|ix| target[ix] - current[ix])
        .min()
        .unwrap()
}

fn part2_find(
    switches: &mut [Switch],
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
                switches.iter().positions(|switch| switch.does_switch(ix)).collect(),
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
            let selected_switch = switches[selected_ix].clone();
            let count = target[joltage_ix] - current[joltage_ix];
            selected_switch.apply(current, count);
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Found single switch to apply {} {} times to get {:?}",
                indent, selected_switch, count, current
            );
            switches[selected_ix].0 = 0; // Effectively disabling that switch
            let result = part2_find(
                switches,
                target,
                current,
                press_count + count as usize,
                #[cfg(feature = "debug_print")]
                &(indent.to_owned() + "  "),
            );
            selected_switch.unapply(current, count);
            switches[selected_ix] = selected_switch; // And undo-ing the last step
            result
        }
        [first_ix, second_ix] => {
            let first_switch = switches[first_ix].clone();
            let second_switch = switches[second_ix].clone();
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Try to apply 2 switches {} total {} times to get {:?}",
                indent, switches, count, current
            );

            let count = target[joltage_ix] - current[joltage_ix];
            let first_max_count = find_max_count(target, current, &first_switch);
            let second_max_count = find_max_count(target, current, &second_switch);
            if first_max_count + second_max_count < count {
                return None;
            }

            first_switch.apply(current, first_max_count);
            second_switch.apply(current, count - first_max_count);
            switches[first_ix].0 = 0;
            switches[second_ix].0 = 0;
            let apply_count = second_max_count - (count - first_max_count);

            let result = (0..=apply_count)
                .filter_map(|ix| {
                    if ix > 0 {
                        first_switch.unapply(current, 1);
                        second_switch.apply(current, 1);
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

            first_switch.unapply(current, first_max_count - apply_count);
            second_switch.unapply(current, second_max_count);
            switches[first_ix] = first_switch;
            switches[second_ix] = second_switch;

            result
        }
        _ => {
            // There are multiple switches we can use, try the most complicated one first
            let &selected_switch_ix = selected_switch_indices
                .iter()
                .max_by_key(|&&ix| switches[ix].0.count_ones())
                .unwrap();
            let selected_switch = switches[selected_switch_ix].clone();
            let max_count = find_max_count(target, current, &selected_switch);
            switches[selected_switch_ix].0 = 0;
            selected_switch.apply(current, max_count + 1);

            let result = (0..=max_count)
                .rev()
                .filter_map(|count| {
                    selected_switch.unapply(current, 1);
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
            switches[selected_switch_ix].0 = selected_switch.0;
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
            let mut switches: Vec<_> = line.switches.iter().map(|&v| Switch(v)).collect();
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
