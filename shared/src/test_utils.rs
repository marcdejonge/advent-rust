use std::fmt::Debug;

#[inline]
pub fn assert_day<Input, O>(input: &Input, calc: fn(&Input) -> O, expected: O)
where
    O: PartialEq + Debug,
    Input: ?Sized,
{
    let result = calc(input);
    assert_eq!(
        result, expected,
        "Expected output of {:?}, but was {:?}",
        expected, result
    );
}

#[allow(clippy::crate_in_macro_def)] // This is the whole point, to use the call-site's crate
#[macro_export]
macro_rules! day_test {
    ( $day: tt, $name: tt => $part1_result: expr) => {
        #[cfg(test)]
        mod $name {
            const INPUT: &[u8] = include_bytes!(concat!(
                "../../input/day",
                stringify!($day),
                "_",
                stringify!($name),
                ".txt"
            ));

            #[test]
            fn part1() {
                let parsed: crate::ParsedInput = advent_lib::parsing::handle_parser_error(INPUT);
                let result = crate::calculate_part1(&parsed);
                assert_eq!(
                    result, $part1_result,
                    concat!(
                        "Expected output of ",
                        stringify!($part1_result),
                        " but was {:?}"
                    ),
                    result
                );
            }
        }
    };
    ( $day: tt, $name: tt => $part1_result: expr, $part2_result: expr) => {
        #[cfg(test)]
        mod $name {
            const INPUT: &[u8] = include_bytes!(concat!(
                "../../input/day",
                stringify!($day),
                "_",
                stringify!($name),
                ".txt"
            ));

            #[test]
            fn part1() {
                let parsed: crate::ParsedInput = advent_lib::parsing::handle_parser_error(INPUT);
                let result = crate::calculate_part1(&parsed);
                assert_eq!(
                    result, $part1_result,
                    concat!(
                        "Expected output of ",
                        stringify!($part1_result),
                        " but was {:?}"
                    ),
                    result
                );
            }

            #[test]
            fn part2() {
                let parsed: crate::ParsedInput = advent_lib::parsing::handle_parser_error(INPUT);
                let result = crate::calculate_part2(&parsed);
                assert_eq!(
                    result, $part2_result,
                    concat!(
                        "Expected output of ",
                        stringify!($part2_result),
                        " but was {:?}"
                    ),
                    result
                );
            }
        }
    };
    ( $day: expr => $part1_result: expr ) => {
        mod full {
            extern crate test;
            use test::Bencher;

            const INPUT: &[u8] =
                include_bytes!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn part1(b: &mut Bencher) {
                let parsed: crate::ParsedInput = advent_lib::parsing::handle_parser_error(INPUT);
                b.iter(|| {
                    let result = crate::calculate_part1(&parsed);
                    assert_eq!(
                        result, $part1_result,
                        concat!(
                            "Expected output of ",
                            stringify!($part1_result),
                            " but was {:?}"
                        ),
                        result
                    );
                })
            }
        }
    };
    ( $day: expr => $part1_result: expr, $part2_result: expr ) => {
        #[cfg(test)]
        mod full {
            extern crate test;

            const INPUT: &[u8] =
                include_bytes!(concat!("../../input/day", stringify!($day), ".txt"));

            #[bench]
            fn parse(b: &mut test::Bencher) {
                b.iter(|| advent_lib::parsing::handle_parser_error::<crate::ParsedInput>(INPUT));
            }

            #[bench]
            fn part1(b: &mut test::Bencher) {
                let parsed: crate::ParsedInput = advent_lib::parsing::handle_parser_error(INPUT);
                b.iter(|| {
                    let result = crate::calculate_part1(&parsed);
                    assert_eq!(
                        result, $part1_result,
                        concat!(
                            "Expected output of ",
                            stringify!($part1_result),
                            " but was {:?}"
                        ),
                        result
                    );
                })
            }

            #[bench]
            fn part2(b: &mut test::Bencher) {
                let parsed: crate::ParsedInput = advent_lib::parsing::handle_parser_error(INPUT);
                b.iter(|| {
                    let result = crate::calculate_part2(&parsed);
                    assert_eq!(
                        result, $part2_result,
                        concat!(
                            "Expected output of ",
                            stringify!($part2_result),
                            " but was {:?}"
                        ),
                        result
                    );
                })
            }
        }
    };
}
