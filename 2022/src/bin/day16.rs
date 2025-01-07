#![feature(test)]
#![feature(map_try_insert)]

use advent_lib::day_main;
use advent_lib::key::Key;
use advent_lib::search::{
    breadth_first_search, depth_first_search, find_max_nonoverlapping_combination,
};
use fxhash::{FxBuildHasher, FxHashMap};
use nom_parse_macros::parse_from;
use std::collections::HashMap;

type Distance = i32;
type FlowRate = i32;

const START_KEY: Key = Key::fixed(b"AA");

struct SewerSystem {
    total_flow_rate: FlowRate,
    valves: FxHashMap<Key, Valve>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[parse_from(tuple(
    preceded("Valve ", Key::parse),
    preceded(" has flow rate=", i32),
    preceded(
        alt("; tunnels lead to valves ", "; tunnel leads to valve "),
        separated_list1(", ", Key::parse)
    ),
))]
struct ParsedValve {
    name: Key,
    rate: FlowRate,
    to: Vec<Key>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve {
    name: Key,
    rate: FlowRate,
    distances: Vec<(Key, Distance)>,
    code: usize,
}

fn find_all_neighbours(
    start: Key,
    valves: &HashMap<Key, ParsedValve, FxBuildHasher>,
) -> Vec<(Key, Distance)> {
    let mut distances: HashMap<Key, Distance, FxBuildHasher> = Default::default();
    distances.insert(start, 0);

    breadth_first_search(
        (start, 0),
        |(current_place, current_dist)| {
            let next_places =
                &valves.get(&current_place).expect("Pointed to a place that doesn't exist").to;
            next_places.iter().map(move |next_place| (*next_place, current_dist + 1))
        },
        |(next_place, new_dist)| distances.try_insert(next_place, new_dist).is_ok(),
    );

    // Optimization: you don't need to travel to any valve that has no flow rate
    valves.values().filter(|v| v.rate == 0).for_each(|v| {
        distances.remove(&v.name);
    });

    let mut distances: Vec<_> = distances.into_iter().collect();

    // Optimization: sort by rate to search for the highest yield first
    distances.sort_by(|(left_key, _left_dist), (right_key, _right_dist)| {
        let left = valves.get(left_key).unwrap();
        let right = valves.get(right_key).unwrap();
        left.rate.cmp(&right.rate)
    });

    distances
}

impl SewerSystem {
    fn neighbours<'a, 'b: 'a>(&'b self, state: State<'a>) -> impl Iterator<Item = State<'a>> {
        state.place.distances.iter().filter_map(move |(next_place, dist)| {
            let next_valve = self.valves.get(next_place).unwrap();
            let new_time_left = state.time_left - dist - 1; // 1 extra time to open the valve
            if new_time_left < 0 || state.open_valves & (1 << next_valve.code) != 0 {
                None
            } else {
                Some(State {
                    place: next_valve,
                    time_left: new_time_left,
                    open_flow_rate: state.open_flow_rate + next_valve.rate,
                    open_valves: state.open_valves | (1 << next_valve.code),
                    total_flow: state.total_flow + new_time_left * next_valve.rate,
                })
            }
        })
    }
}

fn generate_sewer(parsed_valves: Vec<ParsedValve>) -> SewerSystem {
    let valve_map = parsed_valves.iter().map(|pv| (pv.name, pv.clone())).collect();
    let valves: FxHashMap<_, _> = parsed_valves
        .iter()
        // Optimization: don't encode any valve that has no flow-rate, unless it's the started place
        .filter(|pv| pv.rate > 0 || pv.name == START_KEY)
        .enumerate()
        .map(|(code, pv)| {
            (
                pv.name,
                Valve {
                    name: pv.name.clone(),
                    rate: pv.rate,
                    distances: find_all_neighbours(pv.name, &valve_map),
                    code,
                },
            )
        })
        .collect();

    let total_flow_rate: FlowRate = valves.values().map(|v| v.rate).sum();
    SewerSystem { total_flow_rate, valves }
}

fn calculate_part1(sewer: &SewerSystem) -> FlowRate {
    let mut max_state = State::new(sewer.valves.get(&START_KEY).unwrap(), 30);
    depth_first_search(
        max_state.clone(),
        |state| sewer.neighbours(state),
        |state| {
            if state.total_flow > max_state.total_flow {
                max_state = state.clone();
            }
            state.time_left > 1
                    // Optimization: We can stop exploring if there is no potential to improve the current max
                    && state.total_flow
                        + (sewer.total_flow_rate - state.open_flow_rate) * (state.time_left - 2)
                        > max_state.total_flow
        },
    );

    max_state.total_flow
}

fn calculate_part2(sewer: &SewerSystem) -> FlowRate {
    let mut max_total_flows: HashMap<u64, State, FxBuildHasher> = Default::default();
    let mut max_state = State::new(sewer.valves.get(&START_KEY).unwrap(), 26);

    depth_first_search(
        max_state.clone(), // The max was the start
        |state| sewer.neighbours(state),
        |state| {
            if state.total_flow > max_state.total_flow {
                max_state = state.clone();
            }
            if state.total_flow
                > max_total_flows.get(&state.open_valves).map(|it| it.total_flow).unwrap_or(0)
            {
                max_total_flows.insert(state.open_valves, state.clone());
                state.time_left > 1
            } else {
                state.time_left > 1
                    && state.total_flow
                        + (sewer.total_flow_rate - state.open_flow_rate) * (state.time_left - 2)
                        > max_state.total_flow
            }
        },
    );

    let bits_split = (usize::BITS - max_total_flows.len().leading_zeros()) / 2 + 1;
    let (me, elephant) = find_max_nonoverlapping_combination(
        max_total_flows.into_iter(),
        |s| s.total_flow,
        bits_split,
    );
    me.total_flow + elephant.total_flow
}

#[derive(Eq, PartialEq, Clone)]
struct State<'a> {
    place: &'a Valve,
    time_left: i32,
    open_flow_rate: FlowRate,
    open_valves: u64,
    total_flow: FlowRate,
}

impl<'a> State<'a> {
    fn new(start: &'a Valve, time_left: i32) -> State<'a> {
        State { place: start, time_left, open_flow_rate: 0, open_valves: 0, total_flow: 0 }
    }
}

day_main!( generate_sewer => calculate_part1, calculate_part2 );

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 16, example => 1651, 1707 ; generate_sewer );
    day_test!( 16 => 1923, 2594 ; generate_sewer );
}
