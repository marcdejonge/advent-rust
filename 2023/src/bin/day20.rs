#![feature(test)]

use advent_lib::key::Key;
use advent_lib::parsing::separated_lines1;
use advent_lib::*;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use num::integer::lcm;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use Module::*;

const BUTTON: Key = Key::fixed(b"button");
const BROADCASTER: Key = Key::fixed(b"broadcaster");
const RX: Key = Key::fixed(b"rx");

#[parse_from(map(separated_lines1(), parse_modules))]
struct Input {
    initial_state: State,
    reverse_mapping: FxHashMap<Key, Vec<Key>>,
}

#[derive(Debug, Clone)]
struct State {
    modules: FxHashMap<Key, Module>,
}

struct Signal {
    source: Key,
    target: Key,
    is_high: bool,
}

fn signal(source: Key, target: Key, is_high: bool) -> Signal { Signal { source, target, is_high } }

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)?;
        if self.is_high {
            f.write_str(" -high-> ")?;
        } else {
            f.write_str(" -low-> ")?;
        }
        self.source.fmt(f)?;
        Ok(())
    }
}

impl State {
    fn handle_incoming_signal(&mut self, signal: &Signal) -> (Option<&Vec<Key>>, bool) {
        if let Some(module) = self.modules.get_mut(&signal.target) {
            match module {
                Broadcaster { connections, .. } => (Some(connections), false),
                FlipFlop { connections, state, .. } => {
                    if !signal.is_high {
                        *state = !*state;
                        (Some(connections), *state)
                    } else {
                        (None, false)
                    }
                }
                Conjunction { connections, incoming_state, .. } => {
                    incoming_state.insert(signal.source, signal.is_high);
                    let next_signal = !incoming_state.values().all(|b| *b);
                    (Some(connections), next_signal)
                }
            }
        } else {
            (None, false)
        }
    }

    fn button_press<FL, FH>(&mut self, mut when_low: FL, mut when_high: FH)
    where
        FL: FnMut(&Signal),
        FH: FnMut(&Signal),
    {
        let mut signals = VecDeque::with_capacity(1024);
        signals.push_back(signal(BUTTON, BROADCASTER, false));

        while let Some(next_signal) = signals.pop_front() {
            if next_signal.is_high {
                when_high(&next_signal)
            } else {
                when_low(&next_signal)
            }

            let (next_modules, next_is_high) = self.handle_incoming_signal(&next_signal);

            if let Some(next_modules) = next_modules {
                for next_module in next_modules {
                    signals.push_back(signal(next_signal.target, *next_module, next_is_high));
                }
            }
        }
    }
}

#[parse_from]
#[derive(Debug, Clone)]
enum Module {
    #[format(separated_pair(
        value(BROADCASTER, "broadcaster"),
        " -> ",
        separated_list1(", ", Key::parse),
    ))]
    Broadcaster { name: Key, connections: Vec<Key> },
    #[format(separated_pair(
        preceded("%", Key::parse),
        " -> ",
        separated_list1(", ", Key::parse),
    ))]
    FlipFlop {
        name: Key,
        connections: Vec<Key>,
        #[derived(false)]
        state: bool,
    },
    #[format(separated_pair(
        preceded("&", Key::parse),
        " -> ",
        separated_list1(", ", Key::parse),
    ))]
    Conjunction {
        name: Key,
        connections: Vec<Key>,
        #[derived(Default::default())]
        incoming_state: FxHashMap<Key, bool>,
    },
}

impl Module {
    fn connections(&self) -> &[Key] {
        match self {
            Broadcaster { connections, .. } => connections,
            FlipFlop { connections, .. } => connections,
            Conjunction { connections, .. } => connections,
        }
    }

    fn name(&self) -> Key {
        match self {
            Broadcaster { name, .. } => *name,
            FlipFlop { name, .. } => *name,
            Conjunction { name, .. } => *name,
        }
    }
}

fn parse_modules(mut modules: Vec<Module>) -> (State, FxHashMap<Key, Vec<Key>>) {
    let reverse_mapping: FxHashMap<Key, Vec<Key>> = modules
        .iter()
        .flat_map(|module| {
            module.connections().iter().map(move |connection| (*connection, module.name()))
        })
        .fold(FxHashMap::default(), |mut map, (connection, name)| {
            map.entry(connection).or_default().push(name);
            map
        });

    for module in &mut modules {
        if let Conjunction { name, incoming_state, .. } = module {
            reverse_mapping.get(name).unwrap().iter().for_each(|from| {
                incoming_state.insert(*from, false);
            })
        }
    }

    let initial_state =
        State { modules: modules.into_iter().map(|module| (module.name(), module)).collect() };

    (initial_state, reverse_mapping)
}

fn calculate_part1(input: &Input) -> usize {
    let mut state = input.initial_state.clone();
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        state.button_press(|_| low_count += 1, |_| high_count += 1);
    }

    low_count * high_count
}

fn calculate_part2(input: &Input) -> usize {
    let inv = match input.reverse_mapping.get(&RX) {
        Some(slice) => slice.first().unwrap(),
        _ => return 0, // If there is no RX target, then this has no solution
    };
    let triggers = input.reverse_mapping.get(inv).expect("Expected triggers before the inverse");

    let mut state = input.initial_state.clone();
    let mut button_pressed = 0;
    let mut found_numbers = FxHashMap::default();
    loop {
        button_pressed += 1;
        state.button_press(
            |_| {},
            |signal| {
                if triggers.contains(&signal.source) && !found_numbers.contains_key(&signal.source)
                {
                    found_numbers.insert(signal.source, button_pressed);
                }
            },
        );

        if found_numbers.len() == triggers.len() {
            return found_numbers.values().fold(1, |curr, next| lcm(curr, *next));
        }
    }
}

day_main!(Input);
day_test!( 20, simple => 32000000 );
day_test!( 20, example => 11687500 );
day_test!( 20 => 791120136, 215252378794009 );
