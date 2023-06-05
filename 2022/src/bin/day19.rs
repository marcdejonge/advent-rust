#![feature(test)]

use std::convert::identity;
use std::ops::{Add, Index, Sub};

use prse_derive::parse;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::search::depth_first_search;

struct Day {
    blueprints: Vec<Blueprint>,
}

struct Blueprint {
    ix: u32,
    ore_bot_cost: RobotCost,
    clay_bot_cost: RobotCost,
    obsidian_bot_cost: RobotCost,
    geode_bot_cost: RobotCost,
}

#[derive(Copy, Clone)]
struct RobotCost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

impl Blueprint {
    fn calc_max_robots(&self) -> ActiveRobots {
        ActiveRobots {
            ore: identity(self.ore_bot_cost.ore)
                .max(self.clay_bot_cost.ore)
                .max(self.obsidian_bot_cost.ore)
                .max(self.geode_bot_cost.ore),
            clay: identity(self.ore_bot_cost.clay)
                .max(self.clay_bot_cost.clay)
                .max(self.obsidian_bot_cost.clay)
                .max(self.geode_bot_cost.clay),
            obsidian: identity(self.ore_bot_cost.obsidian)
                .max(self.clay_bot_cost.obsidian)
                .max(self.obsidian_bot_cost.obsidian)
                .max(self.geode_bot_cost.obsidian),
            geode: u32::MAX,
        }
    }

    fn get_cost(&self, robot: &Robot) -> &RobotCost {
        match robot {
            Robot::Ore => &self.ore_bot_cost,
            Robot::Clay => &self.clay_bot_cost,
            Robot::Obsidian => &self.obsidian_bot_cost,
            Robot::Geode => &self.geode_bot_cost,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct ActiveRobots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Materials {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add<ActiveRobots> for Materials {
    type Output = Materials;

    fn add(self, rhs: ActiveRobots) -> Self::Output {
        Materials {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Add<&Robot> for ActiveRobots {
    type Output = ActiveRobots;

    fn add(self, rhs: &Robot) -> Self::Output {
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

impl Sub<&RobotCost> for Materials {
    type Output = Materials;

    fn sub(self, rhs: &RobotCost) -> Self::Output {
        Materials {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode,
        }
    }
}

impl Index<&Robot> for ActiveRobots {
    type Output = u32;

    fn index(&self, index: &Robot) -> &Self::Output {
        match index {
            Robot::Ore => &self.ore,
            Robot::Clay => &self.clay,
            Robot::Obsidian => &self.obsidian,
            Robot::Geode => &self.geode,
        }
    }
}

impl Default for ActiveRobots {
    fn default() -> Self { ActiveRobots { ore: 1, clay: 0, obsidian: 0, geode: 0 } }
}

impl Materials {
    fn can_pay_for(&self, other: &RobotCost) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }
}

#[derive(Copy, Clone)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Copy, Clone, Default)]
struct State {
    time: u32,
    materials: Materials,
    active_robots: ActiveRobots,
    bought_robot: Option<Robot>,
    prev_mats: Materials,
}

impl State {
    fn calc_max_geodes(&self) -> u32 {
        self.materials.geode
            + self.time * self.active_robots.geode
            + (self.time * (self.time - 1)) / 2
    }

    fn next(&self) -> State {
        State {
            time: self.time - 1,
            materials: self.materials + self.active_robots,
            active_robots: self.active_robots,
            bought_robot: None,
            prev_mats: self.materials,
        }
    }

    fn next_with_robot(&self, blueprint: &Blueprint, robot: Robot) -> State {
        State {
            time: self.time - 1,
            materials: self.materials + self.active_robots - blueprint.get_cost(&robot),
            active_robots: self.active_robots + &robot,
            bought_robot: Some(robot),
            prev_mats: self.materials,
        }
    }

    fn iter<'a>(
        &self,
        max_robots: &'a ActiveRobots,
        blueprint: &'a Blueprint,
    ) -> StateIterator<'a> {
        StateIterator { from: *self, max_robots, blueprint, robot_index: 0, done: false }
    }
}

const ROBOTS_TO_BUY: [Robot; 4] = [Robot::Geode, Robot::Obsidian, Robot::Clay, Robot::Ore];

struct StateIterator<'a> {
    from: State,
    max_robots: &'a ActiveRobots,
    blueprint: &'a Blueprint,
    robot_index: usize,
    done: bool,
}

impl<'a> Iterator for StateIterator<'a> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.robot_index == 0 {
            self.robot_index += 1;
            if self.from.materials.can_pay_for(self.blueprint.get_cost(&Robot::Geode)) {
                self.done = true; // If we can buy a geode robot, only do that
                return Some(self.from.next_with_robot(self.blueprint, Robot::Geode));
            }
        }

        while self.robot_index < ROBOTS_TO_BUY.len() {
            let robot = ROBOTS_TO_BUY[self.robot_index];
            self.robot_index += 1;
            let cost = self.blueprint.get_cost(&robot);

            if self.from.materials.can_pay_for(cost)
                // Only go up to a maximum count for those robots
                && self.from.active_robots[&robot] < self.max_robots[&robot]
                // If we could have bought it last time, but we didn't, don't buy it now
                && (self.from.bought_robot.is_some() || !self.from.prev_mats.can_pay_for(cost))
            {
                return Some(self.from.next_with_robot(self.blueprint, robot));
            }
        }

        self.done = true;
        Some(self.from.next())
    }
}

fn calculate(blueprint: &Blueprint, start_time: u32) -> State {
    let max_robots = blueprint.calc_max_robots();
    let start = State { time: start_time, ..Default::default() };
    let mut max = start;
    depth_first_search(
        start,
        |from| from.iter(&max_robots, blueprint),
        |state| {
            if state.materials.geode > max.materials.geode {
                max = state;
            }
            state.time > 0 && state.calc_max_geodes() > max.materials.geode
        },
    );
    max
}

impl ExecutableDay for Day {
    type Output = u32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { blueprints: lines.map(|line|{
            let (ix, ore_cost, clay_cost, obs_ore_cost, obs_clay_cost, geode_ore_cost, geode_obs_cost): (u32, u32, u32, u32, u32, u32, u32) = parse!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.");
            Blueprint {
                ix,
                ore_bot_cost: RobotCost { ore: ore_cost, clay: 0, obsidian: 0 },
                clay_bot_cost: RobotCost { ore: clay_cost, clay: 0, obsidian: 0 },
                obsidian_bot_cost: RobotCost { ore: obs_ore_cost, clay: obs_clay_cost, obsidian: 0 },
                geode_bot_cost: RobotCost { ore: geode_ore_cost, clay: 0, obsidian: geode_obs_cost },
            }
        }).collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.blueprints
            .iter()
            .map(|blueprint| blueprint.ix * calculate(blueprint, 24).materials.geode)
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.blueprints.iter().take(3).fold(1, |acc, blueprint| {
            acc * calculate(blueprint, 32).materials.geode
        })
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 19, example => 33, 3472 );
    day_test!( 19 => 1147, 3080 );
}
