use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::vec2::Vec2;
use regex::Regex;
use std::ops::RangeInclusive;

struct Day {
    sensors: Vec<Sensor>,
}

impl Day {
    fn find_all_lines(&self, f: fn(&Vec2<i64>) -> i64) -> Vec<i64> {
        let mut lines_indices = self
            .sensors
            .iter()
            .flat_map(|sensor| {
                let top_left = f(&sensor.location) + 1;
                [top_left + sensor.distance, top_left - sensor.distance]
            })
            .collect::<Vec<_>>();
        lines_indices.sort();
        lines_indices
    }
}

struct Sensor {
    location: Vec2<i64>,
    distance: i64,
}

impl Sensor {
    fn get_overlap(&self, row: i64) -> RangeInclusive<i64> {
        let space = self.distance - (self.location.y - row).abs();
        (self.location.x - space)..=(self.location.x + space)
    }
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let regex = Regex::new(
            "Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)",
        )
        .expect("Regex should be valid");
        Day {
            sensors: iter
                .into_iter()
                .map(|line| {
                    let captures = regex.captures(&line).expect("Line doesn't match regex");
                    let location = Vec2 {
                        x: captures[1].parse().unwrap(),
                        y: captures[2].parse().unwrap(),
                    };
                    let closest_beacon = Vec2 {
                        x: captures[3].parse().unwrap(),
                        y: captures[4].parse().unwrap(),
                    };
                    let distance = location.manhattan_distance(&closest_beacon);
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
                    }
                    if x > valid_range.end {
                        break;
                    }
                    let y = x - down_line;
                    if valid_range.contains(&y)
                        && self.sensors.iter().all(|sensor| {
                            sensor.location.manhattan_distance(&Vec2 { x, y }) > sensor.distance
                        })
                    {
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