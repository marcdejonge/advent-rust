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

pub const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

impl Direction {
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

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            North => 3,
            East => 0,
            South => 1,
            West => 2,
        }
    }
}

impl<T> From<Direction> for Vector<2, T>
where
    T: Zero + One + Neg<Output = T>,
{
    fn from(value: Direction) -> Self { value.as_vec() }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CardinalDirections {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl CardinalDirections {
    pub const ALL: [CardinalDirections; 8] = [
        CardinalDirections::N,
        CardinalDirections::NE,
        CardinalDirections::E,
        CardinalDirections::SE,
        CardinalDirections::S,
        CardinalDirections::SW,
        CardinalDirections::W,
        CardinalDirections::NW,
    ];
}

impl<T> From<CardinalDirections> for Vector<2, T>
where
    T: Zero + One + Neg<Output = T>,
{
    fn from(value: CardinalDirections) -> Self {
        match value {
            CardinalDirections::N => vector2(T::zero(), T::one().neg()),
            CardinalDirections::NE => vector2(T::one(), T::one().neg()),
            CardinalDirections::E => vector2(T::one(), T::zero()),
            CardinalDirections::SE => vector2(T::one(), T::one()),
            CardinalDirections::S => vector2(T::zero(), T::one()),
            CardinalDirections::SW => vector2(T::one().neg(), T::one()),
            CardinalDirections::W => vector2(T::one().neg(), T::zero()),
            CardinalDirections::NW => vector2(T::one().neg(), T::one().neg()),
        }
    }
}
