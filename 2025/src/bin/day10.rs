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
use smallvec::SmallVec;
use std::{array, fmt::Debug};

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

#[derive(Default)]
struct Switches(SmallVec<[Switch; 8]>);

impl Switches {
    fn push(&mut self, switch: Switch) { self.0.push(switch); }
    fn len(&self) -> usize { self.0.len() }
}

impl<'a> IntoIterator for &'a Switches {
    type Item = &'a Switch;
    type IntoIter = std::slice::Iter<'a, Switch>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

fn generate_min_switches(switches: &[Switch]) -> FxHashMap<Lights, Vec<Switches>> {
    let mut min_switches =
        FxHashMap::with_capacity_and_hasher((1 << switches.len()) * 2, Default::default());
    (0u32..(1 << switches.len())).for_each(|switch_mask| {
        let mut selected_switches = Switches::default();
        let mut lights = Lights::default();
        for (ix, &switch) in switches.iter().enumerate() {
            if switch_mask & 1 << ix != 0 {
                selected_switches.push(switch);
                lights = lights.flip(&switch);
            }
        }
        min_switches.entry(lights).or_insert(Vec::new()).push(selected_switches);
    });
    min_switches
}

type Simd = std::simd::u16x16;

#[parse_from(in_braces(map_opt(separated_list1(",", u16), |mut list| {
    if list.len() >= 16 {
        None
    } else {
        while list.len() < 16 {
            list.push(0)
        }
        Some(Simd::from(list.as_array::<16>().unwrap().to_owned()))
    }
})))]
#[derive(Debug, Clone)]
struct Joltages(Simd);

impl Joltages {
    const ZERO: Simd = Simd::splat(0);
    const TWO: Simd = Simd::splat(2);

    fn to_lights(&self) -> Lights {
        Lights(
            self.0.to_array().iter().enumerate().fold(0, |accum, (ix, joltage)| {
                if joltage & 1 != 0 { accum | (1 << ix) } else { accum }
            }),
        )
    }

    fn is_nothing(&self) -> bool { self.0 == Self::ZERO }
    fn half_everything(&mut self) { self.0 /= Self::TWO; }

    fn apply_switch(&mut self, switch: &Switch) -> Option<()> {
        let subtract = Simd::from_array(
            array::try_from_fn(|ix| {
                if switch.switches(ix) {
                    if self.0.as_array()[ix] == 0 {
                        return Err(());
                    }
                    Ok(1)
                } else {
                    Ok(0)
                }
            })
            .ok()?,
        );
        self.0 -= subtract;

        Some(())
    }
}

fn min_presses(
    target: &Joltages,
    min_switches: &FxHashMap<Lights, Vec<Switches>>,
) -> Option<usize> {
    if target.is_nothing() {
        Some(0)
    } else {
        min_switches
            .get(&target.to_lights())?
            .iter()
            .filter_map(|selected_switches| {
                let mut next_target = target.clone();
                for switch in selected_switches {
                    next_target.apply_switch(switch)?;
                }
                next_target.half_everything();
                Some(2 * min_presses(&next_target, min_switches)? + selected_switches.len())
            })
            .min()
    }
}

fn calculate_part2(lines: &[Line]) -> usize {
    lines
        .par_iter()
        .map(|line| {
            let min_switches = generate_min_switches(&line.switches);
            let target = line.joltages.clone();
            min_presses(&target, &min_switches).unwrap()
        })
        .sum()
}

day_main!(Vec<Line>);

day_test!( 10, example => 7, 33 );
day_test!( 10, first25 => 69, 1858 );
day_test!( 10, line54 => 1, 156 );
day_test!( 10 => 425, 15883 );
