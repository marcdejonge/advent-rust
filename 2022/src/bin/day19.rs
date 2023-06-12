#![feature(test)]

use std::mem::transmute;
use std::ops::{Add, Index, Sub};
use std::str::FromStr;

use prse_derive::parse;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::search::depth_first_search;

type Count = u8;

struct Day {
    blueprints: Vec<Blueprint>,
}

struct Blueprint {
    ix: u32,
    ore_bot_cost: RobotCost,
    clay_bot_cost: RobotCost,
    obsidian_bot_cost: RobotCost,
    geode_bot_cost: RobotCost,
    max_robots: ActiveRobots,
}

#[derive(Copy, Clone, Default)]
struct RobotCost {
    ore: Count,
    clay: Count,
    obsidian: Count,
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (ix, ore_cost, clay_cost, obs_ore_cost, obs_clay_cost, geode_ore_cost, geode_obs_cost): (u32, Count, Count, Count, Count, Count, Count)
            = parse!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.");
        Ok(Blueprint {
            ix,
            ore_bot_cost: RobotCost { ore: ore_cost, ..Default::default() },
            clay_bot_cost: RobotCost { ore: clay_cost, ..Default::default() },
            obsidian_bot_cost: RobotCost {
                ore: obs_ore_cost,
                clay: obs_clay_cost,
                ..Default::default()
            },
            geode_bot_cost: RobotCost {
                ore: geode_ore_cost,
                obsidian: geode_obs_cost,
                ..Default::default()
            },
            max_robots: ActiveRobots {
                ore: ore_cost.max(clay_cost).max(obs_ore_cost).max(geode_ore_cost),
                clay: obs_clay_cost,
                obsidian: geode_obs_cost,
                geode: Count::MAX,
            },
        })
    }
}

impl Index<Robot> for Blueprint {
    type Output = RobotCost;

    fn index(&self, index: Robot) -> &Self::Output {
        match index {
            Robot::Ore => &self.ore_bot_cost,
            Robot::Clay => &self.clay_bot_cost,
            Robot::Obsidian => &self.obsidian_bot_cost,
            Robot::Geode => &self.geode_bot_cost,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ActiveRobots {
    ore: Count,
    clay: Count,
    obsidian: Count,
    geode: Count,
}

#[derive(Copy, Clone, Default, Debug)]
struct Materials {
    ore: Count,
    clay: Count,
    obsidian: Count,
    geode: Count,
}

impl Add<ActiveRobots> for Materials {
    type Output = Materials;

    fn add(self, rhs: ActiveRobots) -> Self::Output {
        unsafe {
            let left: u32 = transmute(self);
            let right: u32 = transmute(rhs);
            let sum = left + right; // Assume no overflow
            transmute(sum)
        }
    }
}

impl Add<Robot> for ActiveRobots {
    type Output = ActiveRobots;

    fn add(self, rhs: Robot) -> Self::Output {
        let mut result = self;
        match rhs {
            Robot::Ore => result.ore += 1,
            Robot::Clay => result.clay += 1,
            Robot::Obsidian => result.obsidian += 1,
            Robot::Geode => result.geode += 1,
        }
        result
    }
}

impl Sub<RobotCost> for Materials {
    type Output = Materials;

    fn sub(self, rhs: RobotCost) -> Self::Output {
        Materials {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode,
        }
    }
}

impl Default for ActiveRobots {
    fn default() -> Self { ActiveRobots { ore: 1, clay: 0, obsidian: 0, geode: 0 } }
}

impl Materials {
    fn can_pay_for(&self, other: RobotCost) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }
}

#[derive(Copy, Clone, Debug)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Copy, Clone, Default, Debug)]
struct State {
    time: u8,
    materials: Materials,
    active_robots: ActiveRobots,
}

impl State {
    fn calc_max_geodes(&self) -> Count {
        let bots = self.active_robots.geode as u32;
        let geodes = self.materials.geode as u32;
        let time = self.time as u32;
        let max = geodes + time * bots + (time * (time - 1)) / 2;
        max.clamp(0, Count::MAX as u32) as Count
    }

    fn calc_geodes(&self) -> Count { self.materials.geode + self.active_robots.geode * self.time }

    fn next_robot(&self, cost: RobotCost, robot: Robot) -> Option<State> {
        let mut available = self.materials;
        let mut time_spend = 1u8;
        while !available.can_pay_for(cost) {
            available = available + self.active_robots;
            time_spend += 1;

            if time_spend > self.time {
                return None; // Can't build that robot in time
            }
        }

        Some(State {
            time: self.time - time_spend,
            materials: available + self.active_robots - cost,
            active_robots: self.active_robots + robot,
        })
    }

    fn next_with_ore(&self, blueprint: &Blueprint) -> Option<State> {
        let cost = blueprint.ore_bot_cost;
        if self.active_robots.ore == blueprint.max_robots.ore {
            return None;
        }
        self.next_robot(cost, Robot::Ore)
    }

    fn next_with_clay(&self, blueprint: &Blueprint) -> Option<State> {
        let cost = blueprint.clay_bot_cost;
        if self.active_robots.clay == blueprint.max_robots.clay {
            return None;
        }
        self.next_robot(cost, Robot::Clay)
    }

    fn next_with_obsidian(&self, blueprint: &Blueprint) -> Option<State> {
        let cost = blueprint.obsidian_bot_cost;
        if self.active_robots.obsidian == blueprint.max_robots.obsidian
            || self.active_robots.clay == 0
        // Without clay robots, we won't get any clay to build these
        {
            return None;
        }
        self.next_robot(cost, Robot::Obsidian)
    }

    fn next_with_geode(&self, blueprint: &Blueprint) -> Option<State> {
        let cost = blueprint.geode_bot_cost;
        if self.active_robots.obsidian == 0 {
            // Without any obsidian, we can't get any geode robots
            return None;
        }
        self.next_robot(cost, Robot::Geode)
    }
}

fn calculate(blueprint: &Blueprint, start_time: u8) -> u32 {
    let start = State { time: start_time, ..Default::default() };
    let mut max_geodes = Count::default();
    depth_first_search(
        start,
        |from| {
            [
                from.next_with_geode(blueprint),
                from.next_with_obsidian(blueprint),
                from.next_with_clay(blueprint),
                from.next_with_ore(blueprint),
            ]
            .into_iter()
            .flatten()
        },
        |state| {
            max_geodes = max_geodes.max(state.calc_geodes());
            state.time > 0 && state.calc_max_geodes() > max_geodes
        },
    );
    max_geodes as u32
}

impl ExecutableDay for Day {
    type Output = u32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { blueprints: lines.filter_map(|line| line.parse().ok()).collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.blueprints
            .iter()
            .map(|blueprint| blueprint.ix * calculate(blueprint, 24))
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.blueprints
            .iter()
            .take(3)
            .fold(1, |acc, blueprint| acc * calculate(blueprint, 32))
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    use super::*;

    day_test!( 19, example => 33, 3472 );
    day_test!( 19 => 1147, 3080 );

    #[test]
    fn example_steps_verification() {
        let blueprint: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse().unwrap();
        let state = State { time: 24, ..State::default() };
        let state = state.next_with_clay(&blueprint).unwrap();
        assert_eq!(21, state.time);
        let state = state.next_with_clay(&blueprint).unwrap();
        assert_eq!(19, state.time);
        let state = state.next_with_clay(&blueprint).unwrap();
        assert_eq!(17, state.time);
        let state = state.next_with_obsidian(&blueprint).unwrap();
        assert_eq!(13, state.time);
        let state = state.next_with_clay(&blueprint).unwrap();
        assert_eq!(12, state.time);
        let state = state.next_with_obsidian(&blueprint).unwrap();
        assert_eq!(9, state.time);
        let state = state.next_with_geode(&blueprint).unwrap();
        assert_eq!(6, state.time);
        let state = state.next_with_geode(&blueprint).unwrap();
        assert_eq!(3, state.time);
        assert_eq!(9, state.calc_geodes())
    }
}
