#![feature(test)]

use advent_lib::day_main;
use advent_lib::search::depth_first_search;
use advent_macros::parsable;
use std::mem::transmute;
use std::ops::{Add, Index, Sub};

type Count = u8;

#[parsable(tuple((
    delimited(tag(b"Blueprint "), u32, single(b':')),
    delimited(tag(b" Each ore robot costs "), RobotCost::parser(), single(b'.')),
    delimited(tag(b" Each clay robot costs "), RobotCost::parser(), single(b'.')),
    delimited(tag(b" Each obsidian robot costs "), RobotCost::parser(), single(b'.')),
    delimited(tag(b" Each geode robot costs "), RobotCost::parser(), single(b'.')),
)))]
struct Blueprint {
    ix: u32,
    ore_bot_cost: RobotCost,
    clay_bot_cost: RobotCost,
    obsidian_bot_cost: RobotCost,
    geode_bot_cost: RobotCost,
    #[defer(
        ActiveRobots {
            ore: ore_bot_cost.ore.max(clay_bot_cost.ore).max(obsidian_bot_cost.ore).max(geode_bot_cost.ore),
            clay: obsidian_bot_cost.clay,
            obsidian: geode_bot_cost.obsidian,
            geode: Count::MAX,
        }
    )]
    max_robots: ActiveRobots,
}

#[derive(Copy, Clone, Default)]
#[parsable(tuple((
    map(opt(terminated(u8, tuple((tag(b" ore"), opt(tag(b" and ")))))), Option::unwrap_or_default),
    map(opt(terminated(u8, tuple((tag(b" clay"), opt(tag(b" and ")))))), Option::unwrap_or_default),
    map(opt(terminated(u8, tag(b" obsidian"))), Option::unwrap_or_default),
)))]
struct RobotCost {
    ore: Count,
    clay: Count,
    obsidian: Count,
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
    time: Count,
    materials: Materials,
    active_robots: ActiveRobots,
}

impl State {
    fn calc_max_geodes(&self) -> Count {
        let time = self.time as u32;
        let max = self.calc_geodes() as u32 + (time * (time - 1)) / 2;
        max.clamp(0, Count::MAX as u32) as Count
    }

    fn calc_geodes(&self) -> Count { self.materials.geode + self.active_robots.geode * self.time }

    fn next_robot(&self, cost: RobotCost, robot: Robot) -> Option<State> {
        let mut available = self.materials;
        let mut time_spend = 1;
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

fn calculate(blueprint: &Blueprint, start_time: Count) -> u32 {
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

fn calculate_part1(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints.iter().map(|blueprint| blueprint.ix * calculate(blueprint, 24)).sum()
}

fn calculate_part2(blueprints: &Vec<Blueprint>) -> u32 {
    blueprints
        .iter()
        .take(3)
        .fold(1, |acc, blueprint| acc * calculate(blueprint, 32))
}

day_main!();

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::day_test;
    use advent_lib::parsing::Parsable;
    use nom::Parser;

    day_test!( 19, example => 33, 3472 );
    day_test!( 19 => 1147, 3080 );

    #[test]
    fn example_steps_verification() {
        let blueprint = Blueprint::parser().parse(b"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.").unwrap().1;
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
