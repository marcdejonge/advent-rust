#![feature(test)]

use std::collections::VecDeque;
use std::ops::Shl;

use fxhash::FxHashMap;
use prse_derive::parse;

use advent_lib::day::*;
use BitRule::*;
use Module::*;

type State = i128;

struct Day {
    modules: FxHashMap<String, Module>,
}

impl Day {
    fn step(
        &self,
        state: State,
        module_name: &str,
        signal: bool,
    ) -> (State, Option<&Vec<String>>, bool) {
        if let Some(module) = self.modules.get(module_name) {
            match module {
                Broadcaster { connections } => (state, Some(connections), signal),
                FlipFlop { connections, bit_selector: bit_switch } => {
                    if !signal {
                        let next_state = state ^ bit_switch;
                        let next_signal = (next_state & bit_switch) != 0;
                        (next_state, Some(connections), next_signal)
                    } else {
                        (state, None, signal)
                    }
                }
                Conjunction { connections, bit_rule } => {
                    let next_signal = !bit_rule.check(state);
                    (state, Some(connections), next_signal)
                }
            }
        } else {
            (state, None, signal)
        }
    }

    fn assign_bits_to_flip_flops(&mut self) {
        let mut curr_bit_set = 1;
        self.modules.iter_mut().for_each(|(_, module)| {
            if let FlipFlop { bit_selector: bit_switch, .. } = module {
                *bit_switch = curr_bit_set;
                curr_bit_set = curr_bit_set.shl(1);
                if curr_bit_set == 0 {
                    panic!("Too many flip-flips to represent!")
                }
            }
        });
    }

    fn calculate_conjunction_bits(&mut self, reverse_lookup: FxHashMap<String, Vec<String>>) {
        let mut conjunctions: Vec<String> =
            self.modules
                .iter()
                .filter_map(|(name, module)| {
                    if let Conjunction { .. } = module {
                        Some(name.to_string())
                    } else {
                        None
                    }
                })
                .collect();

        while !conjunctions.is_empty() {
            conjunctions.retain(|name| {
                let mut new_bit_rule = Direct { bit_selector: 0 };

                for source in reverse_lookup.get(name).unwrap() {
                    match self.modules.get(source).unwrap() {
                        Broadcaster { .. } => {}
                        FlipFlop { bit_selector, .. } => {
                            new_bit_rule =
                                new_bit_rule.combine(&Direct { bit_selector: *bit_selector });
                        }
                        Conjunction { bit_rule, .. } => {
                            if matches!(bit_rule, BitRule::Unset) {
                                return true; // Source has not been calculated yet, so do it again next loop
                            }

                            new_bit_rule = new_bit_rule
                                .combine(&Inverse { bit_rule: Box::new(bit_rule.clone()) });
                        }
                    }
                }

                if let Some(Conjunction { bit_rule, .. }) = self.modules.get_mut(name) {
                    *bit_rule = new_bit_rule;
                }

                false // Completed, so we can resume
            });
        }
    }
}

#[derive(Debug)]
enum Module {
    Broadcaster { connections: Vec<String> },
    FlipFlop { connections: Vec<String>, bit_selector: State },
    Conjunction { connections: Vec<String>, bit_rule: BitRule },
}

impl Module {
    fn get_connections(&self) -> &Vec<String> {
        match self {
            Broadcaster { connections } => connections,
            FlipFlop { connections, .. } => connections,
            Conjunction { connections, .. } => connections,
        }
    }
}

#[derive(Debug, Clone)]
enum BitRule {
    Unset,
    Direct { bit_selector: State },
    Inverse { bit_rule: Box<BitRule> },
    And { left: Box<BitRule>, right: Box<BitRule> },
}

impl BitRule {
    fn check(&self, state: State) -> bool {
        match self {
            Unset => false,
            Direct { bit_selector } => (state & bit_selector) == *bit_selector,
            Inverse { bit_rule } => !bit_rule.check(state),
            And { left, right } => left.check(state) && right.check(state),
        }
    }

    fn combine(mut self, other: &BitRule) -> BitRule {
        if let Direct { bit_selector: other_selector } = other {
            if let Direct { bit_selector: self_selector } = &mut self {
                *self_selector |= other_selector;
                return self;
            } else if let And { left, .. } = &mut self {
                if let Direct { bit_selector: self_selector } = &mut **left {
                    *self_selector |= other_selector;
                    return self;
                }
            }
        }

        And { left: Box::new(self), right: Box::new(other.clone()) }
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut reverse_mapping: FxHashMap<String, Vec<String>> = Default::default();
        let modules: FxHashMap<_, _> = lines
            .map(|line| {
                let (mut name, connections): (String, Vec<String>) = parse!(line, "{} -> {:, :}");
                for connection in &connections {
                    if !reverse_mapping.contains_key(connection) {
                        reverse_mapping.insert(connection.clone(), Vec::new());
                    }
                    reverse_mapping.get_mut(connection).unwrap().push(name[1..].to_string());
                }

                if name == "broadcaster" {
                    (name, Broadcaster { connections })
                } else if name.starts_with('%') {
                    name.remove(0);
                    (name, FlipFlop { connections, bit_selector: 0 })
                } else if name.starts_with('&') {
                    name.remove(0);
                    (name, Conjunction { connections, bit_rule: BitRule::Unset })
                } else {
                    panic!("Unknown module variant")
                }
            })
            .collect();

        let mut day = Day { modules };
        day.assign_bits_to_flip_flops();
        day.calculate_conjunction_bits(reverse_mapping);

        day
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut state: State = 0;
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let mut module_trigger = VecDeque::with_capacity(1024);

            #[cfg(test)]
            println!("button -false-> broadcaster");
            module_trigger.push_back(("broadcaster", false));
            low_count += 1;

            while let Some((module_name, signal)) = module_trigger.pop_front() {
                let (next_state, next_modules, next_signal) = self.step(state, module_name, signal);
                if let Some(next_modules) = next_modules {
                    for next_module in next_modules {
                        #[cfg(test)]
                        println!("{module_name} -{next_signal}-> {next_module}");

                        module_trigger.push_back((next_module, next_signal));
                        if next_signal {
                            high_count += 1;
                        } else {
                            low_count += 1;
                        }
                    }
                }
                state = next_state;
            }

            #[cfg(test)]
            println!();
        }

        low_count * high_count
    }

    fn calculate_part2(&self) -> Self::Output {
        if !self
            .modules
            .iter()
            .any(|(_, module)| module.get_connections().iter().any(|to| to == "rx"))
        {
            return 0;
        }

        let mut state: State = 0;
        let mut button_pressed = 0;
        loop {
            let mut module_trigger = VecDeque::with_capacity(1024);

            #[cfg(test)]
            println!("button -false-> broadcaster");
            module_trigger.push_back(("broadcaster", false));
            button_pressed += 1;

            while let Some((module_name, signal)) = module_trigger.pop_front() {
                let (next_state, next_modules, next_signal) = self.step(state, module_name, signal);
                if let Some(next_modules) = next_modules {
                    for next_module in next_modules {
                        #[cfg(test)]
                        println!("{module_name} -{next_signal}-> {next_module}");

                        module_trigger.push_back((next_module, next_signal));

                        if !next_signal && next_module == "rx" {
                            return button_pressed;
                        }
                    }
                }
                state = next_state;
            }

            #[cfg(test)]
            println!();
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 20, simple => 32000000, 0 );
    day_test!( 20, example => 11687500, 0 );
    day_test!( 20 => 791120136, 0 );
}
