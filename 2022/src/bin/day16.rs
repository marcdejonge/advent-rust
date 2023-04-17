#![feature(test)]
#![feature(map_try_insert)]

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::search::{
    breadth_first_search, depth_first_search, find_max_nonoverlapping_combination,
};
use fxhash::FxBuildHasher;
use prse_derive::parse;
use std::collections::HashMap;
use std::str::FromStr;

type NameKey = u16;
type Distance = i32;
type FlowRate = i32;

struct Day {
    total_flow_rate: FlowRate,
    valves: HashMap<NameKey, Valve, FxBuildHasher>,
}

fn encode_string(name: &str) -> NameKey {
    u16::from_ne_bytes(name.as_bytes().try_into().expect("Expected a 2 character name"))
}

const START_KEY: NameKey = u16::from_ne_bytes(*b"AA");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ParsedValve {
    name: String,
    rate: FlowRate,
    to: Vec<String>,
}

impl FromStr for ParsedValve {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = line.split("; ").collect();
        let (name, rate) = parse!(parts[0], "Valve {} has flow rate={}");
        let items = parts[1]
            .strip_prefix("tunnel leads to valve ")
            .or(parts[1].strip_prefix("tunnels lead to valves "))
            .ok_or("Could not find tunnels")?;
        let to = parse!(items, "{:, :}");
        Ok(ParsedValve { name, rate, to })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve {
    name: String,
    rate: FlowRate,
    distances: Vec<(NameKey, Distance)>,
    code: usize,
}

fn find_all_neighbours(
    start: &str,
    valves: &HashMap<NameKey, ParsedValve, FxBuildHasher>,
) -> Vec<(NameKey, Distance)> {
    let mut distances: HashMap<NameKey, Distance, FxBuildHasher> = Default::default();
    let start_key = encode_string(start);
    distances.insert(start_key, 0);

    breadth_first_search(
        (start_key, 0),
        |(current_place, current_dist)| {
            let next_places =
                &valves.get(&current_place).expect("Pointed to a place that doesn't exist").to;
            next_places
                .iter()
                .map(move |next_place| (encode_string(next_place), current_dist + 1))
        },
        |(next_place, new_dist)| distances.try_insert(next_place, new_dist).is_ok(),
    );

    // Optimization: you don't need to travel to any valve that has no flow rate
    valves.values().filter(|v| v.rate == 0).for_each(|v| {
        distances.remove(&encode_string(&v.name));
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

impl Day {
    fn neighbours<'a, 'b: 'a>(&'b self, state: State<'a>) -> impl Iterator<Item = State<'a>> {
        state.place.distances.iter().filter_map(move |(next_place, dist)| {
            let next_valve = self.valves.get(next_place).unwrap();
            let new_time_left = state.time_left - dist - 1; // 1 extra time to open the valve
            return if new_time_left < 0 || state.open_valves & (1 << next_valve.code) != 0 {
                None
            } else {
                Some(State {
                    place: next_valve,
                    time_left: new_time_left,
                    open_flow_rate: state.open_flow_rate + next_valve.rate,
                    open_valves: state.open_valves | (1 << next_valve.code),
                    total_flow: state.total_flow + new_time_left * next_valve.rate,
                })
            };
        })
    }
}

impl ExecutableDay for Day {
    type Output = FlowRate;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let parsed_valves: HashMap<_, _, FxBuildHasher> = lines
            .filter_map(|line| line.parse().ok())
            .map(|pv: ParsedValve| (encode_string(&pv.name), pv))
            .collect();
        let valves = parsed_valves
            .values()
            // Optimization: don't encode any valve that has no flow-rate, unless it's the started place
            .filter(|pv| pv.rate > 0 || pv.name == "AA")
            .enumerate()
            .map(|(code, pv)| {
                (
                    encode_string(&pv.name),
                    Valve {
                        name: pv.name.clone(),
                        rate: pv.rate,
                        distances: find_all_neighbours(pv.name.as_str(), &parsed_valves),
                        code,
                    },
                )
            })
            .collect::<HashMap<_, _, _>>();

        let total_flow_rate: FlowRate = valves.values().map(|v| v.rate).sum();
        Day { total_flow_rate, valves }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut max_state = State::new(self.valves.get(&START_KEY).unwrap(), 30);
        depth_first_search(
            max_state.clone(),
            |state| self.neighbours(state),
            |state| {
                if state.total_flow > max_state.total_flow {
                    max_state = state.clone();
                }
                state.time_left > 1
                    // Optimization: We can stop exploring if there is no potential to improve the current max
                    && state.total_flow
                        + (self.total_flow_rate - state.open_flow_rate) * (state.time_left - 2)
                        > max_state.total_flow
            },
        );

        max_state.total_flow
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut max_total_flows: HashMap<u64, State, FxBuildHasher> = Default::default();
        let mut max_state = State::new(self.valves.get(&START_KEY).unwrap(), 26);

        depth_first_search(
            max_state.clone(), // The max was the start
            |state| self.neighbours(state),
            |state| {
                if state.total_flow > max_state.total_flow {
                    max_state = state.clone();
                }
                if state.total_flow
                    > max_total_flows.get(&state.open_valves).map(|it| it.total_flow).unwrap_or(0)
                {
                    max_total_flows.insert(state.open_valves.clone(), state.clone());
                    state.time_left > 1
                } else {
                    state.time_left > 1
                        && state.total_flow
                            + (self.total_flow_rate - state.open_flow_rate) * (state.time_left - 2)
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

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 16, example => 1651, 1707 );
    day_test!( 16 => 1923, 2594 );
}
