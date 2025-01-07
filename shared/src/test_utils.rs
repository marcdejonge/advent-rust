use std::fmt::Debug;

#[inline]
pub fn assert_day<Input, O>(input: &Input, calc: fn(&Input) -> O, expected: O)
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
    ( $day: tt, $name: tt => $part1_result: expr) => {
        day_test!( $day, $name => $part1_result ; std::convert::identity );
    };
    ( $day: tt, $name: tt => $part1_result: expr ; $prepare: path ) => {
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
                let parsed = $prepare(advent_lib::parsing::handle_parser_error(input));
                advent_lib::test_utils::assert_day(&parsed, calculate_part1, $part1_result);
            }
        }
    };
    ( $day: tt, $name: tt => $part1_result: expr, $part2_result: expr) => {
        day_test!( $day, $name => $part1_result, $part2_result ; std::convert::identity );
    };
    ( $day: tt, $name: tt => $part1_result: expr, $part2_result: expr ; $prepare: path ) => {
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
                let parsed = $prepare(advent_lib::parsing::handle_parser_error(input));
                advent_lib::test_utils::assert_day(&parsed, calculate_part1, $part1_result);
            }

            #[test]
            fn part2() {
                let parsed = $prepare(advent_lib::parsing::handle_parser_error(input));
                advent_lib::test_utils::assert_day(&parsed, calculate_part2, $part2_result);
            }
        }
    };
    ( $day: expr => $part1_result: expr ) => {
        day_test!( $day => $part1_result ; std::convert::identity );
    };
    ( $day: expr => $part1_result: expr ; $prepare: path ) => {
        mod full {
            extern crate test;
            use super::super::*;
            use test::Bencher;

            const input: &[u8] =
                include_bytes!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let parsed = $prepare(advent_lib::parsing::handle_parser_error(input));
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&parsed, calculate_part1, $part1_result);
                })
            }
        }
    };
    ( $day: expr => $part1_result: expr, $part2_result: expr ) => {
        day_test!( $day => $part1_result, $part2_result ; std::convert::identity );
    };
    ( $day: expr => $part1_result: expr, $part2_result: expr ; $prepare: path ) => {
        mod full {
            extern crate test;
            use super::super::*;
            use test::Bencher;

            const input: &[u8] =
                include_bytes!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let parsed = $prepare(advent_lib::parsing::handle_parser_error(input));
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&parsed, calculate_part1, $part1_result);
                })
            }

            #[bench]
            fn part2(b: &mut Bencher) {
                let parsed = $prepare(advent_lib::parsing::handle_parser_error(input));
                b.iter(|| {
                    advent_lib::test_utils::assert_day(&parsed, calculate_part2, $part2_result);
                })
            }
        }
    };
}
