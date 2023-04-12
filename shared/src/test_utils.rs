use crate::day::ExecutableDay;
use std::fmt::Debug;

pub fn assert_day<Day: ExecutableDay>(
    input: &str,
    expected_part1: Day::Output,
    expected_part2: Day::Output,
) where
    Day: FromIterator<String>,
    Day::Output: PartialEq + Debug,
{
    let day: Day = input.lines().map(|line| line.to_owned()).collect();
    assert_eq!(day.calculate_part1(), expected_part1);
    assert_eq!(day.calculate_part2(), expected_part2);
}

#[macro_export]
macro_rules! day_test {
    ( $day: tt, $name: tt => $part1_result: expr, $part2_result: expr ) => {
        #[test]
        fn $name() {
            advent_lib::test_utils::assert_day::<super::Day>(
                include_str!(concat!(
                    "../../input/day",
                    stringify!($day),
                    "_",
                    stringify!($name),
                    ".txt"
                )),
                $part1_result,
                $part2_result,
            );
        }
    };
    ( $day: expr => $part1_result: expr, $part2_result: expr ) => {
        extern crate test;
        use super::*;
        use test::Bencher;

        #[bench]
        fn full(b: &mut Bencher) {
            b.iter(|| {
                advent_lib::test_utils::assert_day::<super::Day>(
                    include_str!(concat!("../../input/day", stringify!($day), ".txt")),
                    $part1_result,
                    $part2_result,
                );
            })
        }
    };
}
