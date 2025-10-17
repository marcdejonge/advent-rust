use crate::direction::CardinalDirection::*;
use crate::direction::Direction::*;
use crate::geometry::{vector2, Vector};
use nom::character::complete::one_of;
use nom::error::{ErrorKind, ParseError};
use nom::{AsChar, Compare, IResult, Input};
use nom_parse_trait::ParseFrom;
use num_traits::{One, Zero};
use std::ops::Neg;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'N' | b'U' | b'^' | b'3' | 3 => North,
            b'E' | b'R' | b'>' | b'0' | 0 => East,
            b'S' | b'D' | b'v' | b'1' | 1 => South,
            b'W' | b'L' | b'<' | b'2' | 2 => West,
            _ => panic!("Illegal byte representation for a Direction: {value}"),
        }
    }
}

impl<I, E> ParseFrom<I, E> for Direction
where
    E: ParseError<I>,
    I: Input,
    <I as Input>::Item: AsChar + Copy,
    I: for<'a> Compare<&'a [u8]>,
    for<'a> &'a str: nom::FindToken<<I as Input>::Item>,
{
    fn parse(input: I) -> IResult<I, Self, E> {
        if input.input_len() == 0 {
            return Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)));
        }

        let (rest, direction) = one_of("NESWURDL^>v<0123")(input.clone())?;

        let direction = match direction {
            'N' | 'U' | '^' | '3' => North,
            'E' | 'R' | '>' | '0' => East,
            'S' | 'D' | 'v' | '1' => South,
            'W' | 'L' | '<' | '2' => West,
            _ => return Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::OneOf))),
        };

        Ok((rest, direction))
    }
}

impl Direction {
    pub const ALL: [Direction; 4] = [North, East, South, West];

    pub const fn turn_left(self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub const fn turn_right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub const fn is_horizontal(self) -> bool { matches!(self, East | West) }

    pub const fn is_vertical(self) -> bool { matches!(self, North | South) }

    pub fn as_vec<T>(self) -> Vector<2, T>
    where
        T: Zero + One + Neg<Output = T>,
    {
        match self {
            North => vector2(T::zero(), T::one().neg()),
            East => vector2(T::one(), T::zero()),
            South => vector2(T::zero(), T::one()),
            West => vector2(T::one().neg(), T::zero()),
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

macro_rules! direction_into_nr {
    () => {};
    ($nr_type:ty) => {
        impl From<Direction> for $nr_type {
            fn from(value: Direction) -> Self {
                match value {
                    North => 3,
                    East => 0,
                    South => 1,
                    West => 2,
                }
            }
        }
    };
    ($nr_type:ty, $($types:ty),*) => {
        direction_into_nr!($nr_type);
        direction_into_nr!($($types),*);
    };
}

direction_into_nr!(i8, i16, i32, i64, u8, u16, u32, u64, usize);

impl<T> From<Direction> for Vector<2, T>
where
    T: Zero + One + Neg<Output = T>,
{
    fn from(value: Direction) -> Self { value.as_vec() }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CardinalDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl CardinalDirection {
    pub const ALL: [CardinalDirection; 8] = [N, NE, E, SE, S, SW, W, NW];
}

impl<T> From<CardinalDirection> for Vector<2, T>
where
    T: Zero + One + Neg<Output = T>,
{
    fn from(value: CardinalDirection) -> Self {
        match value {
            N => vector2(T::zero(), T::one().neg()),
            NE => vector2(T::one(), T::one().neg()),
            E => vector2(T::one(), T::zero()),
            SE => vector2(T::one(), T::one()),
            S => vector2(T::zero(), T::one()),
            SW => vector2(T::one().neg(), T::one()),
            W => vector2(T::one().neg(), T::zero()),
            NW => vector2(T::one().neg(), T::one().neg()),
        }
    }
}

impl From<CardinalDirection> for usize {
    fn from(value: CardinalDirection) -> Self {
        match value {
            N => 0,
            NE => 1,
            E => 2,
            SE => 3,
            S => 4,
            SW => 5,
            W => 6,
            NW => 7,
        }
    }
}
