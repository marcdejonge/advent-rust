#![feature(test)]
#![feature(portable_simd)]
#![feature(slice_as_array)]
#![feature(array_try_from_fn)]

use advent_lib::{parsing::*, *};
use bit_set::BitSet;
use fxhash::FxHashMap;
use itertools::{Itertools, iterate};
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use std::{fmt::Debug, simd::*};

#[parse_from(({}, delimited(space0, separated_list1(space1, {}), space0), {}))]
#[derive(Debug)]
struct Line {
    target_lights: Lights,
    switches: Vec<Switch>,
    joltages: Joltages,
}

#[parse_from(in_parens(map(separated_list1(",", u32), |nrs| {
    nrs.iter().fold(0, |accum, nr| {
        accum | (1 << nr)
    })
})))]
#[derive(Debug, Clone, Copy)]
struct Switch(u32);

impl Switch {
    fn switches(&self, ix: usize) -> bool { self.0 & (1 << ix) != 0 }
}

#[parse_from(in_brackets(map(is_a(".#"), |input: I| {
    input.as_bytes().iter().enumerate().fold(0, |accum, (ix, b)| {
        accum | if *b == b'#' { 1 << ix } else { 0 }
    })
})))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Lights(u32);

impl Lights {
    fn flip(&self, switch: &Switch) -> Self { Lights(self.0 ^ switch.0) }
}

struct LightsSet(BitSet);

impl LightsSet {
    fn new() -> Self {
        let mut bs = BitSet::new();
        bs.insert(0);
        Self(bs)
    }

    fn iter(&self) -> impl Iterator<Item = Lights> { self.0.iter().map(|ix| Lights(ix as u32)) }

    fn contains(&self, lights: Lights) -> bool { self.0.contains(lights.0 as usize) }
}

impl FromIterator<Lights> for LightsSet {
    fn from_iter<T: IntoIterator<Item = Lights>>(iter: T) -> Self {
        LightsSet(iter.into_iter().map(|lights| lights.0 as usize).collect())
    }
}

fn calculate_part1(lines: &[Line]) -> usize {
    lines
        .par_iter()
        .filter_map(|line| {
            let mut initial = BitSet::new();
            initial.insert(0);
            iterate(LightsSet::new(), |last| {
                last.iter()
                    .flat_map(|lights| line.switches.iter().map(move |s| lights.flip(s)))
                    .collect()
            })
            .find_position(|nrs| nrs.contains(line.target_lights))
            .map(|(depth, _)| depth)
        })
        .sum()
}

const LANES: usize = 16;

#[derive(Default)]
struct Switches {
    joltages: Simd<u16, LANES>,
    len: usize,
}

impl Switches {
    fn add(&self, switch: Switch) -> Self {
        let mut joltages = self.joltages;
        joltages
            .as_mut_array()
            .iter_mut()
            .enumerate()
            .filter(|&(ix, _)| switch.switches(ix))
            .for_each(|(_, v)| *v += 1);
        Self { joltages, len: self.len + 1 }
    }
}

struct SwitchesSelection(FxHashMap<Lights, Vec<Switches>>);

impl SwitchesSelection {
    pub fn from(switches: &[Switch]) -> Self {
        let mut min_switches = Self(FxHashMap::with_capacity_and_hasher(
            (1 << switches.len()) * 2,
            Default::default(),
        ));
        min_switches.add_switch_from(switches, Default::default(), Default::default());
        min_switches
    }

    fn add_switch_from(
        &mut self,
        switches: &[Switch],
        selected_switches: Switches,
        lights: Lights,
    ) {
        if let [next_switch, rest_switches @ ..] = switches {
            // First, try with adding this switch
            self.add_switch_from(
                rest_switches,
                selected_switches.add(*next_switch),
                lights.flip(next_switch),
            );
            // Second try without selecting this switch
            self.add_switch_from(rest_switches, selected_switches, lights);
        } else {
            // No more switches to add, just push the result
            self.0.entry(lights).or_default().push(selected_switches);
        }
    }

    #[inline]
    fn get(&self, lights: Lights) -> Option<&[Switches]> {
        self.0.get(&lights).map(|vec| vec.as_slice())
    }
}

#[parse_from(in_braces(map(separated_list1(",", u16), |v| Simd::load_or_default(&v))))]
#[derive(Default, Debug, Clone)]
struct Joltages(Simd<u16, LANES>);

impl Joltages {
    fn to_lights(&self) -> Lights {
        Lights(
            self.0.to_array().iter().enumerate().fold(0, |accum, (ix, joltage)| {
                if joltage & 1 != 0 { accum | (1 << ix) } else { accum }
            }),
        )
    }

    const TWO: Simd<u16, LANES> = Simd::splat(2);

    fn min_presses(&self, switches_selection: &SwitchesSelection) -> Option<usize> {
        switches_selection
            .get(self.to_lights())?
            .iter()
            .filter_map(|selected_switches| {
                if self.0 == selected_switches.joltages {
                    Some(selected_switches.len)
                } else if selected_switches
                    .joltages
                    .as_array()
                    .iter()
                    .zip(self.0.as_array().iter())
                    .any(|(left, right)| left > right)
                {
                    None
                } else {
                    let mut next_target = Joltages(self.0 - selected_switches.joltages);
                    next_target.0 /= Self::TWO;
                    Some(2 * next_target.min_presses(switches_selection)? + selected_switches.len)
                }
            })
            .min()
    }
}

fn calculate_part2(lines: &[Line]) -> usize {
    lines
        .par_iter()
        .map(|line| line.joltages.min_presses(&SwitchesSelection::from(&line.switches)).unwrap())
        .sum()
}

day_main!(Vec<Line>);

day_test!( 10, example => 7, 33 );
day_test!( 10, first25 => 69, 1858 );
day_test!( 10, line54 => 1, 156 );
day_test!( 10 => 425, 15883 );
