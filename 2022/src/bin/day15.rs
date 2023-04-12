#![feature(test)]
use std::ops::RangeInclusive;

use prse_derive::parse;
use rusttype::Point;

use advent_lib::day::{execute_day, ExecutableDay};

struct Day {
    sensors: Vec<Sensor>,
}

impl Day {
    fn find_all_lines(&self, f: fn(Point<i64>) -> i64) -> Vec<i64> {
        let mut lines_indices = self
            .sensors
            .iter()
            .flat_map(|sensor| {
                let top_left = f(sensor.location) + 1;
                [top_left + sensor.distance, top_left - sensor.distance]
            })
            .collect::<Vec<_>>();
        lines_indices.sort();
        lines_indices
    }

    fn contains(&self, place: Point<i64>) -> bool {
        self.sensors.iter().any(|sensor| sensor.contains(place))
    }
}

struct Sensor {
    location: Point<i64>,
    distance: i64,
}

impl Sensor {
    fn get_overlap(&self, row: i64) -> RangeInclusive<i64> {
        let space = self.distance - (self.location.y - row).abs();
        (self.location.x - space)..=(self.location.x + space)
    }

    fn contains(&self, place: Point<i64>) -> bool {
        manhattan_distance(self.location, place) <= self.distance
    }
}

fn manhattan_distance(from: Point<i64>, to: Point<i64>) -> i64 {
    let vector = to - from;
    vector.x.abs() + vector.y.abs()
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Day {
            sensors: iter
                .into_iter()
                .map(|line| {
                    let (loc_x, loc_y, beacon_x, beacon_y) = parse!(
                        line,
                        "Sensor at x={}, y={}: closest beacon is at x={}, y={}"
                    );
                    let location = Point { x: loc_x, y: loc_y };
                    let closest_beacon = Point { x: beacon_x, y: beacon_y };
                    let distance = manhattan_distance(closest_beacon, location);
                    Sensor { location, distance }
                })
                .collect(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = i64;

    fn calculate_part1(&self) -> Self::Output {
        let check_row: i64 = if self.sensors.len() < 20 { 10 } else { 2000000 };
        let ranges: Vec<_> =
            self.sensors.iter().map(|sensor| sensor.get_overlap(check_row)).collect();
        ranges.iter().map(|range| range.end()).max().unwrap()
            - ranges.iter().map(|range| range.start()).min().unwrap()
    }

    fn calculate_part2(&self) -> Self::Output {
        let valid_range = if self.sensors.len() < 20 { 0..20 } else { 0..4000000 };
        let down_lines = self.find_all_lines(|v| v.x - v.y);
        let up_lines = self.find_all_lines(|v| v.x + v.y);

        for &down_line in &down_lines {
            for &up_line in &up_lines {
                if (down_line + up_line) % 2 == 0 {
                    let x = (down_line + up_line) / 2;
                    if x < 0 {
                        continue;
                    } else if x > valid_range.end {
                        break;
                    }
                    let y = x - down_line;
                    if valid_range.contains(&y) && !self.contains(Point { x, y }) {
                        return x * 4000000 + y;
                    }
                }
            }
        }

        panic!("No solution has been found")
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 15, example => 26, 56000011 );
    day_test!( 15 => 4725496, 12051287042458 );
}
