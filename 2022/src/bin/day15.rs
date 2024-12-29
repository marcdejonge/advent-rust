#![feature(test)]
#![allow(clippy::ptr_arg)]

use advent_lib::day_main;
use advent_lib::geometry::point2;
use advent_macros::parsable;
use std::ops::RangeInclusive;

type Point = advent_lib::geometry::Point<2, i64>;

fn find_all_lines(sensors: &[Sensor], f: fn(Point) -> i64) -> Vec<i64> {
    let mut lines_indices = sensors
        .iter()
        .flat_map(|sensor| {
            let top_left = f(sensor.sensor) + 1;
            [top_left + sensor.distance, top_left - sensor.distance]
        })
        .collect::<Vec<_>>();
    lines_indices.sort();
    lines_indices
}

fn contains(sensors: &[Sensor], place: Point) -> bool {
    sensors.iter().any(|sensor| sensor.contains(place))
}

#[parsable(tuple((
    map(
        tuple((preceded(tag("Sensor at x="), i64), preceded(tag(", y="), i64))),
        |(x,y)| point2(x, y)
    ),
    map(
        tuple((preceded(tag(": closest beacon is at x="), i64), preceded(tag(", y="), i64))),
        |(x,y)| point2(x, y)
    ),
)))]
struct Sensor {
    sensor: Point,
    #[intermediate]
    beacon: Point,
    #[defer((sensor - beacon).euler())]
    distance: i64,
}

impl Sensor {
    fn get_overlap(&self, row: i64) -> RangeInclusive<i64> {
        let space = self.distance - (self.sensor.y() - row).abs();
        (self.sensor.x() - space)..=(self.sensor.x() + space)
    }

    fn contains(&self, place: Point) -> bool { (self.sensor - place).euler() <= self.distance }
}

fn calculate_part1(sensors: &Vec<Sensor>) -> i64 {
    let check_row: i64 = if sensors.len() < 20 { 10 } else { 2000000 };
    let ranges: Vec<_> = sensors.iter().map(|sensor| sensor.get_overlap(check_row)).collect();
    ranges.iter().map(|range| range.end()).max().unwrap()
        - ranges.iter().map(|range| range.start()).min().unwrap()
}

fn calculate_part2(sensors: &Vec<Sensor>) -> i64 {
    let valid_range = if sensors.len() < 20 { 0..20 } else { 0..4000000 };
    let down_lines = find_all_lines(sensors, |v| v.x() - v.y());
    let up_lines = find_all_lines(sensors, |v| v.x() + v.y());

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
                if valid_range.contains(&y) && !contains(sensors, point2(x, y)) {
                    return x * 4000000 + y;
                }
            }
        }
    }

    panic!("No solution has been found")
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 15, example => 26, 56000011 );
    day_test!( 15 => 4725496, 12051287042458 );
}
