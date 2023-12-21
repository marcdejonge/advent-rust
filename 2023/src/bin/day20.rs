#![feature(test)]

use std::collections::VecDeque;

use fxhash::FxHashMap;
use num::integer::lcm;
use prse::*;

use advent_lib::day::*;
use advent_lib::key::Key;
use Module::*;

const BUTTON: Key = Key::fixed(b"button");
const BROADCASTER: Key = Key::fixed(b"broadcaster");
const RX: Key = Key::fixed(b"rx");

struct Day {
    initial_state: State,
    reverse_mapping: FxHashMap<Key, Vec<Key>>,
}

#[derive(Debug, Clone)]
struct State {
    modules: FxHashMap<Key, Module>,
}

impl State {
    fn handle_incoming_signal(
        &mut self,
        previous_key: Key,
        signal: bool,
        current_key: Key,
    ) -> (Option<&Vec<Key>>, bool) {
        if let Some(module) = self.modules.get_mut(&current_key) {
            match module {
                Broadcaster { connections } => (Some(connections), signal),
                FlipFlop { connections, state } => {
                    if !signal {
                        *state = !*state;
                        (Some(connections), *state)
                    } else {
                        (None, false)
                    }
                }
                Conjunction { connections, incoming_state } => {
                    incoming_state.insert(previous_key, signal);
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
        FL: FnMut(Key, Key),
        FH: FnMut(Key, Key),
    {
        let mut signals = VecDeque::with_capacity(1024);
        signals.push_back((BUTTON, false, BROADCASTER));

        while let Some((from, signal, to)) = signals.pop_front() {
            if signal {
                when_high(from, to)
            } else {
                when_low(from, to)
            }

            let (next_modules, next_signal) = self.handle_incoming_signal(from, signal, to);

            if let Some(next_modules) = next_modules {
                for next_module in next_modules {
                    signals.push_back((to, next_signal, *next_module));
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster { connections: Vec<Key> },
    FlipFlop { connections: Vec<Key>, state: bool },
    Conjunction { connections: Vec<Key>, incoming_state: FxHashMap<Key, bool> },
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut reverse_mapping: FxHashMap<Key, Vec<Key>> = Default::default();
        let mut modules: FxHashMap<Key, Module> = lines
            .map(|line| {
                let (name, connections): (String, Vec<Key>) = parse!(line, "{} -> {:, :}");

                for connection in &connections {
                    if !reverse_mapping.contains_key(connection) {
                        reverse_mapping.insert(*connection, Vec::new());
                    }
                    reverse_mapping
                        .get_mut(connection)
                        .unwrap()
                        .push(Key::from_str(&name[1..]).unwrap());
                }

                if name == "broadcaster" {
                    (BROADCASTER, Broadcaster { connections })
                } else if let Some(name) = name.strip_prefix('%') {
                    let name = Key::from_str(name).unwrap();
                    (name, FlipFlop { connections, state: false })
                } else if let Some(name) = name.strip_prefix('&') {
                    let name = Key::from_str(name).unwrap();
                    (
                        name,
                        Conjunction { connections, incoming_state: Default::default() },
                    )
                } else {
                    panic!("Unknown module variant")
                }
            })
            .collect();

        for (name, module) in &mut modules {
            if let Conjunction { incoming_state, .. } = module {
                reverse_mapping.get(name).unwrap().iter().for_each(|from| {
                    incoming_state.insert(*from, false);
                })
            }
        }

        Day { initial_state: State { modules }, reverse_mapping }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut state = self.initial_state.clone();
        let mut low_count = 0;
        let mut high_count = 0;

        for _ in 0..1000 {
            state.button_press(|_, _| low_count += 1, |_, _| high_count += 1);
        }

        low_count * high_count
    }

    fn calculate_part2(&self) -> Self::Output {
        let inv = match self.reverse_mapping.get(&RX) {
            Some(slice) => slice.first().unwrap(),
            _ => return 0, // If there is no RX target, then this has no solution
        };
        let triggers = self.reverse_mapping.get(inv).expect("Expected triggers before the inverse");

        let mut state = self.initial_state.clone();
        let mut button_pressed = 0;
        let mut found_numbers = FxHashMap::default();
        loop {
            button_pressed += 1;
            state.button_press(
                |_, _| {},
                |key, _| {
                    if triggers.contains(&key) && !found_numbers.contains_key(&key) {
                        found_numbers.insert(key, button_pressed);
                    }
                },
            );

            if found_numbers.len() == triggers.len() {
                return found_numbers.values().fold(1, |curr, next| lcm(curr, *next));
            }
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 20, simple => 32000000 );
    day_test!( 20, example => 11687500 );
    day_test!( 20 => 791120136, 215252378794009 );
}
