#![feature(test)]

use advent_lib::{iter_utils::IteratorUtils, *};
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use std::{fmt::Debug, ops::Deref, sync::Arc};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[parse_from]
enum SnailNumber {
    #[format({})]
    Single(u64),
    #[format(match "[{},{}]")]
    Pair(Arc<SnailNumber>, Arc<SnailNumber>),
}

impl Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(v) => write!(f, "{}", v),
            Self::Pair(l, r) => write!(f, "[{:?},{:?}]", l.deref(), r.deref()),
        }
    }
}

impl SnailNumber {
    fn pair_with(self, other: SnailNumber) -> Self {
        Self::Pair(Arc::new(self), Arc::new(other))
    }

    fn reduce(self) -> SnailNumber {
        let mut current = self;
        while let Some(next) = current.reduce_step() {
            current = next;
        }
        current
    }

    fn reduce_step(&self) -> Option<SnailNumber> {
        self.reduce_deep_pair(0)
            .map(|(_, next, _)| next)
            .or_else(|| self.reduce_big_number())
    }

    fn reduce_deep_pair(&self, depth: usize) -> Option<(u64, SnailNumber, u64)> {
        match self {
            &Self::Single(_) => None,
            Self::Pair(left, right) => {
                if depth >= 4
                    && let Self::Single(left) = left.deref()
                    && let Self::Single(right) = right.deref()
                {
                    Some((*left, Self::Single(0), *right))
                } else if let Some((l_add, cloned, r_add)) = left.reduce_deep_pair(depth + 1) {
                    if r_add == 0 {
                        Some((l_add, Self::Pair(Arc::new(cloned), right.clone()), 0))
                    } else {
                        Some((l_add, cloned.pair_with(right.add_from_left(r_add)), 0))
                    }
                } else if let Some((l_add, cloned, r_add)) = right.reduce_deep_pair(depth + 1) {
                    if l_add == 0 {
                        Some((0, Self::Pair(left.clone(), Arc::new(cloned)), r_add))
                    } else {
                        Some((0, left.add_from_right(l_add).pair_with(cloned), r_add))
                    }
                } else {
                    None
                }
            }
        }
    }

    fn add_from_left(&self, value: u64) -> Self {
        match self {
            &Self::Single(val) => Self::Single(val + value),
            Self::Pair(left, right) => {
                Self::Pair(Arc::new(left.add_from_left(value)), right.clone())
            }
        }
    }

    fn add_from_right(&self, value: u64) -> Self {
        match self {
            &Self::Single(val) => Self::Single(val + value),
            Self::Pair(left, right) => {
                Self::Pair(left.clone(), Arc::new(right.add_from_right(value)))
            }
        }
    }

    fn reduce_big_number(&self) -> Option<Self> {
        match self {
            &Self::Single(val) if val >= 10 => {
                Some(Self::Single(val / 2).pair_with(Self::Single(val.div_ceil(2))))
            }
            &Self::Single(_) => None,
            Self::Pair(left, right) => left
                .reduce_big_number()
                .map(|left| Self::Pair(Arc::new(left), right.clone()))
                .or_else(|| {
                    right
                        .reduce_big_number()
                        .map(|right| Self::Pair(left.clone(), Arc::new(right)))
                }),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            &Self::Single(val) => val,
            Self::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

fn calculate_part1(nrs: &[SnailNumber]) -> u64 {
    nrs.iter()
        .cloned()
        .reduce(|curr, next| curr.pair_with(next).reduce())
        .unwrap()
        .magnitude()
}

fn calculate_part2(nrs: &[SnailNumber]) -> u64 {
    nrs.iter()
        .cloned()
        .combinations::<2>()
        .par_bridge()
        .flat_map(|[left, right]| {
            [
                left.clone().pair_with(right.clone()).reduce().magnitude(),
                right.pair_with(left).reduce().magnitude(),
            ]
        })
        .max()
        .unwrap()
}

day_main!(Vec<SnailNumber>);

day_test!( 18, example => 4140, 3993 );
day_test!( 18 => 4323, 4749 );

#[cfg(test)]
mod test {
    use crate::*;
    use nom_parse_trait::ParseFrom;

    #[inline]
    fn parse(value: &str) -> SnailNumber {
        ParseFrom::<_, ()>::parse(value)
            .expect("Could not parse input")
            .1
    }

    #[inline]
    fn assert_step(nr: SnailNumber, next: &str, expected: &str) -> SnailNumber {
        let next = nr.pair_with(parse(next)).reduce();
        assert_eq!(parse(expected), next);
        next
    }

    #[test]
    fn test_reduce_left() {
        assert_eq!(
            parse("[[[[0,9],2],3],4]"),
            parse("[[[[[9,8],1],2],3],4]").reduce()
        )
    }

    #[test]
    fn test_reduct_right() {
        assert_eq!(
            parse("[7,[6,[5,[7,0]]]]"),
            parse("[7,[6,[5,[4,[3,2]]]]]").reduce()
        )
    }

    #[test]
    fn test_reduce() {
        let init = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let mut nr = init.reduce_step().unwrap();
        assert_eq!(parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"), nr);
        nr = nr.reduce_step().unwrap();
        assert_eq!(parse("[[[[0,7],4],[15,[0,13]]],[1,1]]"), nr);
        nr = nr.reduce_step().unwrap();
        assert_eq!(parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"), nr);
        nr = nr.reduce_step().unwrap();
        assert_eq!(parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"), nr);
        nr = nr.reduce_step().unwrap();
        assert_eq!(parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), nr);
    }

    #[test]
    fn test_simple() {
        let mut nr = parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        nr = assert_step(nr, "[5,5]", "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        nr = assert_step(nr, "[6,6]", "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_bigger() {
        let mut nr = parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        nr = assert_step(
            nr,
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
        nr = assert_step(
            nr,
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
        );
        nr = assert_step(
            nr,
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
        );
        nr = assert_step(
            nr,
            "[7,[5,[[3,8],[1,4]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
        );
        nr = assert_step(
            nr,
            "[[2,[2,2]],[8,[8,1]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
        );
        nr = assert_step(
            nr,
            "[2,9]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
        );
        nr = assert_step(
            nr,
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
        );
        nr = assert_step(
            nr,
            "[[[5,[7,4]],7],1]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
        );
        nr = assert_step(
            nr,
            "[[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }
}
