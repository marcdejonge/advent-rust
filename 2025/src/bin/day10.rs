#![feature(test)]
#![feature(portable_simd)]
#![feature(slice_as_array)]
#![feature(array_try_from_fn)]

use advent_lib::{builder::with, parsing::*, *};
use bit_set::BitSet;
use fxhash::FxHashMap;
use itertools::{Itertools, iterate};
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use smallvec::SmallVec;
use std::{array, fmt::Debug, simd::*};

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

#[derive(Default, Clone)]
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

fn add_min_switches(
    switches: &[Switch],
    map: &mut FxHashMap<Lights, Vec<Switches>>,
    selected_switches: Switches,
    lights: Lights,
) {
    if switches.is_empty() {
        map.entry(lights).or_default().push(selected_switches);
    } else {
        // First, try with adding this switch
        let next_switch = switches[0];
        add_min_switches(
            &switches[1..],
            map,
            with(selected_switches.clone(), |it| it.push(next_switch)),
            lights.flip(&next_switch),
        );
        // Second try without selecting this switch
        add_min_switches(&switches[1..], map, selected_switches, lights);
    }
}

fn generate_min_switches(switches: &[Switch]) -> FxHashMap<Lights, Vec<Switches>> {
    with(
        FxHashMap::with_capacity_and_hasher((1 << switches.len()) * 2, Default::default()),
        |map| add_min_switches(switches, map, Default::default(), Default::default()),
    )
}

#[parse_from(in_braces(separated_list1(",", u16)))]
#[derive(Debug, Clone)]
struct Joltages(Vec<u16>);

impl Joltages {
    fn as_target<const N: usize>(&self) -> JoltageTarget<N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let mut array = [0; N];
        self.0.iter().enumerate().for_each(|(ix, v)| array[ix] = *v);
        JoltageTarget(Simd::from_array(array))
    }

    fn min_presses(&self, min_switches: &FxHashMap<Lights, Vec<Switches>>) -> Option<usize> {
        match self.0.len() {
            0..=4 => self.as_target::<4>().min_presses(min_switches),
            5..=8 => self.as_target::<8>().min_presses(min_switches),
            9..=16 => self.as_target::<16>().min_presses(min_switches),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct JoltageTarget<const N: usize>(Simd<u16, N>)
where
    LaneCount<N>: SupportedLaneCount;

impl<const N: usize> JoltageTarget<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Simd<u16, N> = Simd::splat(0);
    const TWO: Simd<u16, N> = Simd::splat(2);

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

    fn min_presses(&self, min_switches: &FxHashMap<Lights, Vec<Switches>>) -> Option<usize> {
        if self.is_nothing() {
            Some(0)
        } else {
            min_switches
                .get(&self.to_lights())?
                .iter()
                .filter_map(|selected_switches| {
                    let mut next_target = self.clone();
                    for switch in selected_switches {
                        next_target.apply_switch(switch)?;
                    }
                    next_target.half_everything();
                    Some(2 * next_target.min_presses(min_switches)? + selected_switches.len())
                })
                .min()
        }
    }
}

fn calculate_part2(lines: &[Line]) -> usize {
    lines
        .par_iter()
        .map(|line| line.joltages.min_presses(&generate_min_switches(&line.switches)).unwrap())
        .sum()
}

day_main!(Vec<Line>);

day_test!( 10, example => 7, 33 );
day_test!( 10, first25 => 69, 1858 );
day_test!( 10, line54 => 1, 156 );
day_test!( 10 => 425, 15883 );
