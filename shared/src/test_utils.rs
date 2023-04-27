use crate::day::ExecutableDay;
use std::fmt::Debug;

pub fn assert_day<Day: ExecutableDay>(
    input: &str,
    expected_part1: Day::Output,
    expected_part2: Day::Output,
) where
    Day::Output: PartialEq + Debug,
{
    let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
    assert_eq!(
        day.calculate_part1(),
        expected_part1,
        "Part 1 was expected to be {:?}",
        expected_part1
    );
    assert_eq!(
        day.calculate_part2(),
        expected_part2,
        "Part 2 was expected to be {:?}",
        expected_part2
    );
}

#[macro_export]
macro_rules! day_test {
    ( $day: tt, $name: tt => $part1_result: expr, $part2_result: expr ) => {
        mod $name {
            use super::super::*;

            const input: &str = include_str!(concat!(
                "../../input/day",
                stringify!($day),
                "_",
                stringify!($name),
                ".txt"
            ));

            #[test]
            fn part1() {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                assert_eq!(day.calculate_part1(), $part1_result);
            }

            #[test]
            fn part2() {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                assert_eq!(day.calculate_part2(), $part2_result);
            }
        }
    };
    ( $day: expr => $part1_result: expr, $part2_result: expr ) => {
        mod full {
            extern crate test;
            use super::super::*;
            use test::Bencher;

            const input: &str = include_str!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                b.iter(|| {
                    assert_eq!(day.calculate_part1(), $part1_result);
                })
            }

            #[bench]
            fn part2(b: &mut Bencher) {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                b.iter(|| {
                    assert_eq!(day.calculate_part2(), $part2_result);
                })
            }
        }
    };
}
