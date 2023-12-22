use std::fmt::Debug;

use crate::day::ExecutableDay;

#[inline]
pub fn assert_day<Day: ExecutableDay>(
    input: &Day,
    calc: fn(&Day) -> Day::Output,
    expected: Day::Output,
) where
    Day::Output: PartialEq + Debug,
{
    let result = calc(input);
    assert_eq!(
        result, expected,
        "Expected output of {:?}, but was {:?}",
        expected, result
    );
}

#[macro_export]
macro_rules! day_test {
    ( $day: tt, $name: tt => $part1_result: expr ) => {
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
                advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
            }
        }
    };
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
                advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
            }

            #[test]
            fn part2() {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                advent_lib::test_utils::assert_day(&day, Day::calculate_part2, $part2_result);
            }
        }
    };
    ( $day: expr => $part1_result: expr ) => {
        mod full {
            extern crate test;
            use super::super::*;
            use test::Bencher;

            const input: &str = include_str!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
                })
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
                    advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
                })
            }

            #[bench]
            fn part2(b: &mut Bencher) {
                let day = Day::from_lines(input.lines().map(|line| line.to_owned()));
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&day, Day::calculate_part2, $part2_result);
                })
            }
        }
    };
}
