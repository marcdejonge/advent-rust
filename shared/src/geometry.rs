use crate::traits::NotEq;
use num_traits::One;
use std::fmt::{Debug, Formatter};
use std::ops;
use std::str::FromStr;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Point<const D: usize, T> {
    pub coords: [T; D],
}

impl<const D: usize, T> FromStr for Point<D, T>
where
    T: FromStr + Default + Copy,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Point { coords: parse_coords(s)? }) }
}

impl<const D: usize, T> Debug for Point<D, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { debug(&self.coords, "Point", f) }
}

pub const fn point2<T>(x: T, y: T) -> Point<2, T> { Point { coords: [x, y] } }
pub const fn point3<T>(x: T, y: T, z: T) -> Point<3, T> { Point { coords: [x, y, z] } }
pub const fn point4<T>(w: T, x: T, y: T, z: T) -> Point<4, T> { Point { coords: [w, x, y, z] } }

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Vector<const D: usize, T> {
    pub coords: [T; D],
}

impl<const D: usize, T> FromStr for Vector<D, T>
where
    T: FromStr + Default + Copy,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Vector { coords: parse_coords(s)? }) }
}

impl<const D: usize, T> Debug for Vector<D, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { debug(&self.coords, "Vector", f) }
}

pub const fn vector2<T>(x: T, y: T) -> Vector<2, T> { Vector { coords: [x, y] } }
pub const fn vector3<T>(x: T, y: T, z: T) -> Vector<3, T> { Vector { coords: [x, y, z] } }
pub const fn vector4<T>(w: T, x: T, y: T, z: T) -> Vector<4, T> { Vector { coords: [w, x, y, z] } }

pub fn unit_vector<const D: usize, T: Copy + One>() -> Vector<D, T> {
    Vector { coords: [T::one(); D] }
}

fn parse_coords<const D: usize, T>(s: &str) -> Result<[T; D], String>
where
    T: Default + Copy + FromStr,
{
    let mut coords: [T; D] = [Default::default(); D];
    for (ix, r) in s.split(',').map(|p| p.parse::<T>()).enumerate() {
        match r {
            Ok(c) => {
                let coord = coords
                    .get_mut(ix)
                    .ok_or_else(|| format!("Invalid index {ix}, matching for {D}"))?;
                *coord = c;
            }
            Err(_) => return Err(format!("Could not parse component {ix}")),
        }
    }
    Ok(coords)
}

fn debug<const D: usize, T: Debug>(
    coords: &[T; D],
    t: &str,
    f: &mut Formatter,
) -> std::fmt::Result {
    f.write_str(t)?;
    f.write_str("(")?;
    for ix in 0..D {
        if ix > 0 {
            f.write_str(",")?;
        }
        coords[ix].fmt(f)?;
    }
    f.write_str(")")?;
    Ok(())
}

// point + vector -> point
impl<const D: usize, T: ops::Add<Output = T> + Copy> ops::Add<Vector<D, T>> for Point<D, T> {
    type Output = Point<D, T>;
    fn add(self, rhs: Vector<D, T>) -> Point<D, T> {
        let mut coords = self.coords;
        for (ix, v) in rhs.coords.into_iter().enumerate() {
            coords[ix] = coords[ix] + v;
        }
        Point { coords }
    }
}

// vector + vector -> vector
impl<const D: usize, T: ops::Add<Output = T> + Copy> ops::Add for Vector<D, T> {
    type Output = Vector<D, T>;
    fn add(self, rhs: Vector<D, T>) -> Vector<D, T> {
        let mut coords = self.coords;
        for (ix, v) in rhs.coords.into_iter().enumerate() {
            coords[ix] = coords[ix] + v;
        }
        Vector { coords }
    }
}

// point - point -> vector
impl<const D: usize, T: ops::Sub<Output = T> + Copy> ops::Sub for Point<D, T> {
    type Output = Vector<D, T>;
    fn sub(self, rhs: Point<D, T>) -> Vector<D, T> {
        let mut coords = self.coords;
        for (ix, v) in rhs.coords.into_iter().enumerate() {
            coords[ix] = coords[ix] - v;
        }
        Vector { coords }
    }
}

// point - vector -> point
impl<const D: usize, T: ops::Sub<Output = T> + Copy> ops::Sub<Vector<D, T>> for Point<D, T> {
    type Output = Point<D, T>;
    fn sub(self, rhs: Vector<D, T>) -> Point<D, T> {
        let mut coords = self.coords;
        for (ix, v) in rhs.coords.into_iter().enumerate() {
            coords[ix] = coords[ix] - v;
        }
        Point { coords }
    }
}

// vector - vector -> vector
impl<const D: usize, T: ops::Sub<Output = T> + Copy> ops::Sub for Vector<D, T> {
    type Output = Vector<D, T>;
    fn sub(self, rhs: Vector<D, T>) -> Vector<D, T> {
        let mut coords = self.coords;
        for (ix, v) in rhs.coords.into_iter().enumerate() {
            coords[ix] = coords[ix] - v;
        }
        Vector { coords }
    }
}

// vector * const = vector
impl<const D: usize, T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vector<D, T> {
    type Output = Vector<D, T>;
    fn mul(self, rhs: T) -> Vector<D, T> {
        let mut coords = self.coords;
        for ix in 0..D {
            coords[ix] = coords[ix] * rhs;
        }
        Vector { coords }
    }
}

impl<const D: usize, F, T> From<Vector<D, F>> for Vector<D, T>
where
    F: Into<T>,
    (F, T): NotEq,
{
    fn from(value: Vector<D, F>) -> Self { Vector { coords: value.coords.map(F::into) } }
}

impl<const D: usize, T: ops::Mul<Output = T> + One> Vector<D, T> {
    pub fn content_size(self) -> T {
        self.coords.into_iter().fold(T::one(), |acc, next| acc * next)
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Rect<const D: usize, T> {
    pub min: Point<D, T>,
    pub max: Point<D, T>,
}

impl<const D: usize, T: ops::Sub<Output = T> + Copy> Rect<D, T> {
    pub fn total_size(&self) -> Vector<D, T> { self.max - self.min }
}

impl<const D: usize, T> Rect<D, T>
where
    T: PartialOrd,
{
    pub fn contains_inclusive(&self, point: &Point<D, T>) -> bool {
        for ix in 0..D {
            if point.coords[ix] < self.min.coords[ix] || point.coords[ix] > self.max.coords[ix] {
                return false;
            }
        }
        return true;
    }
}

pub trait RectFind<const D: usize, T> {
    fn enclosing_rect(self) -> Option<Rect<D, T>>;
}

impl<const D: usize, T: PartialOrd, I> RectFind<D, T> for I
where
    T: PartialOrd + Copy,
    I: Iterator<Item = Point<D, T>>,
{
    fn enclosing_rect(mut self) -> Option<Rect<D, T>> {
        let mut min = self.next()?.clone();
        let mut max = min.clone();

        loop {
            match self.next() {
                None => break,
                Some(p) => {
                    for ix in 0..D {
                        if p.coords[ix] < min.coords[ix] {
                            min.coords[ix] = p.coords[ix];
                        }
                        if p.coords[ix] > max.coords[ix] {
                            max.coords[ix] = p.coords[ix];
                        }
                    }
                }
            }
        }

        Some(Rect { min, max })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add_vector() {
        assert_eq!(point2(3, 3), point2(1, 2) + vector2(2, 1));
        assert_eq!(point3(-6, 6, 0), point3(-12, 12, 1) + vector3(6, -6, -1));
        assert_eq!(point4(1, 2, 3, 4), point4(0, 0, 0, 0) + vector4(1, 2, 3, 4));
    }

    #[test]
    fn vector_add_vector() { assert_eq!(vector2(4, 4), vector2(1, 2) + vector2(3, 2)) }

    #[test]
    fn point_sub_point() { assert_eq!(vector2(-1, 2), point2(4, 2) - point2(5, 0)) }

    #[test]
    fn vector_sub_vector() { assert_eq!(vector2(0, 0), vector2(2, 4) - vector2(2, 4)) }

    #[test]
    fn multiply_vector() { assert_eq!(vector2(6, 6), vector2(2, 2) * 3) }
}
