use std::fmt::Debug;

use crate::day::ExecutableDay;

#[inline]
pub fn assert_day<Day: ExecutableDay, O>(input: &Day, calc: fn(&Day) -> O, expected: O)
where
    O: PartialEq + Debug,
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

            const input: &[u8] = include_bytes!(concat!(
                "../../input/day",
                stringify!($day),
                "_",
                stringify!($name),
                ".txt"
            ));

            #[test]
            fn part1() {
                let day = advent_lib::parsing::handle_parser_error(input, Day::parser());
                advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
            }
        }
    };
    ( $day: tt, $name: tt => $part1_result: expr, $part2_result: expr ) => {
        mod $name {
            use super::super::*;

            const input: &[u8] = include_bytes!(concat!(
                "../../input/day",
                stringify!($day),
                "_",
                stringify!($name),
                ".txt"
            ));

            #[test]
            fn part1() {
                let day = advent_lib::parsing::handle_parser_error(input, Day::parser());
                advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
            }

            #[test]
            fn part2() {
                let day = advent_lib::parsing::handle_parser_error(input, Day::parser());
                advent_lib::test_utils::assert_day(&day, Day::calculate_part2, $part2_result);
            }
        }
    };
    ( $day: expr => $part1_result: expr ) => {
        mod full {
            extern crate test;
            use super::super::*;
            use test::Bencher;

            const input: &[u8] =
                include_bytes!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let day = advent_lib::parsing::handle_parser_error(input, Day::parser());
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

            const input: &[u8] =
                include_bytes!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let day = advent_lib::parsing::handle_parser_error(input, Day::parser());
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&day, Day::calculate_part1, $part1_result);
                })
            }

            #[bench]
            fn part2(b: &mut Bencher) {
                let day = advent_lib::parsing::handle_parser_error(input, Day::parser());
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&day, Day::calculate_part2, $part2_result);
                })
            }
        }
    };
}
