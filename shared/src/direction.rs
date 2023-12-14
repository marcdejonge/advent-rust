use std::ops::Neg;

use num_traits::{One, Zero};

use crate::direction::Direction::*;
use crate::geometry::{vector2, Vector};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
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
