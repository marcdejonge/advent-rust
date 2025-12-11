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
            accum | if *b == b'#' { 1 << ix } else { 0 }
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

struct SwitchFinder {
    target: Vec<u32>,
    switches: Vec<Switch>,
    current: Vec<u32>,
}

impl SwitchFinder {
    fn new(line: &Line) -> Self {
        Self {
            target: line.joltages.clone(),
            switches: line.switches.iter().map(|&v| Switch(v)).collect(),
            current: vec![0; line.joltages.len()],
        }
    }

    fn hit_target(&self) -> bool { self.target == self.current }
    fn overshot_target(&self) -> bool {
        self.current.iter().zip(self.target.iter()).any(|(curr, tar)| curr > tar)
    }

    fn find_max_count(&self, switch: &Switch) -> u32 {
        (0..self.target.len())
            .filter(|&ix| switch.does_switch(ix))
            .map(|ix| self.target[ix] - self.current[ix])
            .min()
            .unwrap()
    }

    fn remove_switch(&mut self, ix: usize) -> Switch {
        let curr = &mut self.switches[ix];
        let switch = curr.clone();
        curr.0 = 0;
        switch
    }

    fn restore_switch(&mut self, ix: usize, switch: Switch) { self.switches[ix].0 = switch.0; }

    fn find(
        &mut self,
        press_count: usize,
        #[cfg(feature = "debug_print")] indent: &str,
    ) -> Option<usize> {
        if self.hit_target() {
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Found target {:?} after {} presses",
                indent, self.target, press_count
            );
            return Some(press_count);
        } else if self.overshot_target() {
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Overshot the target {:?} with {:?}",
                indent, self.target, self.current
            );
            return None;
        } else {
            #[cfg(feature = "debug_print")]
            eprintln!(
                "{}Finding for part2({:?}, {:?}, {:?}, {})",
                indent, self.switches, self.target, self.current, press_count
            );
        }

        let (joltage_ix, selected_switch_indices) = (0..self.target.len())
            .filter(|&ix| self.current[ix] < self.target[ix])
            .map(|ix| {
                (
                    ix,
                    self.switches.iter().positions(|switch| switch.does_switch(ix)).collect(),
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
                let selected_switch = self.remove_switch(selected_ix);
                let count = self.target[joltage_ix] - self.current[joltage_ix];
                selected_switch.apply(self.current.as_mut_slice(), count);
                #[cfg(feature = "debug_print")]
                eprintln!(
                    "{}Found single switch to apply {} {} times to get {:?}",
                    indent, selected_switch, count, current
                );
                let result = self.find(
                    press_count + count as usize,
                    #[cfg(feature = "debug_print")]
                    &(indent.to_owned() + "  "),
                );
                selected_switch.unapply(self.current.as_mut_slice(), count);
                self.restore_switch(selected_ix, selected_switch);
                result
            }
            [first_ix, second_ix] => {
                let first_switch = self.switches[first_ix].clone();
                let second_switch = self.switches[second_ix].clone();
                #[cfg(feature = "debug_print")]
                eprintln!(
                    "{}Try to apply 2 switches {} total {} times to get {:?}",
                    indent, switches, count, current
                );

                let count = self.target[joltage_ix] - self.current[joltage_ix];
                let first_max_count = self.find_max_count(&first_switch);
                let second_max_count = self.find_max_count(&second_switch);
                if first_max_count + second_max_count < count {
                    return None;
                }

                first_switch.apply(self.current.as_mut_slice(), first_max_count);
                second_switch.apply(self.current.as_mut_slice(), count - first_max_count);
                self.switches[first_ix].0 = 0;
                self.switches[second_ix].0 = 0;
                let apply_count = second_max_count - (count - first_max_count);

                let result = (0..=apply_count)
                    .filter_map(|ix| {
                        if ix > 0 {
                            first_switch.unapply(self.current.as_mut_slice(), 1);
                            second_switch.apply(self.current.as_mut_slice(), 1);
                        }

                        self.find(
                            press_count + count as usize,
                            #[cfg(feature = "debug_print")]
                            &(indent.to_owned() + "  "),
                        )
                    })
                    .min();

                first_switch.unapply(self.current.as_mut_slice(), first_max_count - apply_count);
                second_switch.unapply(self.current.as_mut_slice(), second_max_count);
                self.switches[first_ix] = first_switch;
                self.switches[second_ix] = second_switch;

                result
            }
            _ => {
                // There are multiple switches we can use, try the most complicated one first
                let &selected_switch_ix = selected_switch_indices
                    .iter()
                    .max_by_key(|&&ix| self.switches[ix].0.count_ones())
                    .unwrap();
                let selected_switch = self.switches[selected_switch_ix].clone();
                let max_count = self.find_max_count(&selected_switch);
                self.switches[selected_switch_ix].0 = 0;
                selected_switch.apply(self.current.as_mut_slice(), max_count + 1);

                let result = (0..=max_count)
                    .rev()
                    .filter_map(|count| {
                        selected_switch.unapply(self.current.as_mut_slice(), 1);
                        #[cfg(feature = "debug_print")]
                        eprintln!(
                            "{}Trying to apply switch {} {} times to get {:?}",
                            indent, selected_switch, count, current
                        );
                        self.find(
                            press_count + count as usize,
                            #[cfg(feature = "debug_print")]
                            &(indent.to_owned() + "  "),
                        )
                    })
                    .min();
                self.switches[selected_switch_ix].0 = selected_switch.0;
                result
            }
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
            let result = SwitchFinder::new(line)
                .find(
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
day_test!( 10, first25 => 69, 1858 );
day_test!( 10 => 425 ); // 15883 for the second part, but it's way too slow for testing right now
