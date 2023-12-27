use std::cmp::min;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Index, Mul, Neg, Sub};
use std::str::FromStr;

use num_traits::{abs, One, Signed};
use prse::*;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Point<const D: usize, T> {
    pub coords: [T; D],
}

impl<const D: usize, T> FromStr for Point<D, T>
where
    T: FromStr + Default + Copy,
    <T as FromStr>::Err: Display,
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

impl<const D: usize, T> Default for Point<D, T>
where
    T: Default + Copy,
{
    fn default() -> Self { Point { coords: [T::default(); D] } }
}

impl<T> Point<2, T>
where
    T: Copy,
{
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
}

impl<T> Point<3, T>
where
    T: Copy,
{
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
    pub fn z(&self) -> T { self.coords[2] }
}

impl<T> Point<4, T>
where
    T: Copy,
{
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
    pub fn z(&self) -> T { self.coords[2] }
    pub fn w(&self) -> T { self.coords[3] }
}

impl<const D: usize, T> Point<D, T> {
    pub fn switch_dimensions<const R: usize>(&self) -> Point<R, T>
    where
        T: Default + Copy,
    {
        let mut result = Point::default();
        for ix in 0..min(D, R) {
            result.coords[ix] = self.coords[ix];
        }
        result
    }
}

pub const fn point2<T>(x: T, y: T) -> Point<2, T> { Point { coords: [x, y] } }
pub const fn point3<T>(x: T, y: T, z: T) -> Point<3, T> { Point { coords: [x, y, z] } }
pub const fn point4<T>(w: T, x: T, y: T, z: T) -> Point<4, T> { Point { coords: [w, x, y, z] } }

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Vector<const D: usize, T> {
    pub coords: [T; D],
}

impl<const D: usize, T, R> From<[T; D]> for Vector<D, R>
where
    T: Into<R>,
{
    fn from(value: [T; D]) -> Self { Vector { coords: value.map(T::into) } }
}

impl<const D: usize, T> FromStr for Vector<D, T>
where
    T: FromStr + Default + Copy,
    <T as FromStr>::Err: Display,
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

impl<const D: usize, T> Default for Vector<D, T>
where
    T: Default + Copy,
{
    fn default() -> Self { Vector { coords: [T::default(); D] } }
}

impl<const D: usize, T> Vector<D, T> {
    pub fn switch_dimensions<const R: usize>(&self) -> Vector<R, T>
    where
        T: Default + Copy,
    {
        let mut result = Vector::default();
        for ix in 0..min(D, R) {
            result.coords[ix] = self.coords[ix];
        }
        result
    }
}

impl<const D: usize, T> Index<usize> for Vector<D, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output { &self.coords[index] }
}

impl<const D: usize, T> Index<usize> for Point<D, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output { &self.coords[index] }
}

impl<T> Vector<2, T>
where
    T: Copy,
{
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }

    pub fn euler(&self) -> T
    where
        T: Signed,
    {
        abs(self.coords[0]) + abs(self.coords[1])
    }
}

impl<T> Vector<3, T>
where
    T: Copy,
{
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
    pub fn z(&self) -> T { self.coords[2] }
}

impl<T> Vector<4, T>
where
    T: Copy,
{
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
    pub fn z(&self) -> T { self.coords[2] }
    pub fn w(&self) -> T { self.coords[3] }
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
    <T as FromStr>::Err: Display,
{
    let mut coords: [T; D] = [Default::default(); D];
    for (ix, r) in s.split(',').map(|p| p.trim().parse::<T>()).enumerate() {
        match r {
            Ok(c) => {
                if let Some(coord) = coords.get_mut(ix) {
                    *coord = c;
                } else {
                    return Err(format!("Invalid index {ix}, matching for {D}"));
                }
            }
            Err(e) => return Err(format!("Could not parse component {ix}: {e}")),
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
    for (ix, coord) in coords.iter().enumerate() {
        if ix > 0 {
            f.write_str(",")?;
        }
        coord.fmt(f)?;
    }
    f.write_str(")")?;
    Ok(())
}

// point + vector -> point
impl<const D: usize, T: Add<Output = T> + Copy> Add<Vector<D, T>> for Point<D, T> {
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
impl<const D: usize, T: Add<Output = T> + Copy> Add for Vector<D, T> {
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
impl<const D: usize, T: Sub<Output = T> + Copy> Sub for Point<D, T> {
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
impl<const D: usize, T: Sub<Output = T> + Copy> Sub<Vector<D, T>> for Point<D, T> {
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
impl<const D: usize, T: Sub<Output = T> + Copy> Sub for Vector<D, T> {
    type Output = Vector<D, T>;
    fn sub(self, rhs: Vector<D, T>) -> Vector<D, T> {
        let mut coords = self.coords;
        for (ix, v) in rhs.coords.into_iter().enumerate() {
            coords[ix] = coords[ix] - v;
        }
        Vector { coords }
    }
}

// point * const = point
impl<const D: usize, T: Mul<Output = T> + Copy> Mul<T> for Point<D, T> {
    type Output = Point<D, T>;
    fn mul(self, rhs: T) -> Point<D, T> { Point { coords: self.coords.map(|c| c * rhs) } }
}

// vector * const = vector
impl<const D: usize, T: Mul<Output = T> + Copy> Mul<T> for Vector<D, T> {
    type Output = Vector<D, T>;
    fn mul(self, rhs: T) -> Vector<D, T> { Vector { coords: self.coords.map(|c| c * rhs) } }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vector<2, T> {
    pub fn cross(self, rhs: Vector<2, T>) -> T { rhs.x() * self.y() - self.x() * rhs.y() }
}

impl<const D: usize, T: Mul<Output = T> + One> Vector<D, T> {
    pub fn content_size(self) -> T {
        self.coords.into_iter().fold(T::one(), |acc, next| acc * next)
    }
}

impl<const D: usize, T: Neg<Output = T>> Neg for Vector<D, T> {
    type Output = Vector<D, T>;

    fn neg(self) -> Self::Output { Vector { coords: self.coords.map(|x| x.neg()) } }
}

impl<const D: usize, T> From<Vector<D, T>> for Point<D, T> {
    fn from(value: Vector<D, T>) -> Self { Point { coords: value.coords } }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BoundingBox<const D: usize, T> {
    min: Point<D, T>,
    max: Point<D, T>,
}

impl<const D: usize, T> BoundingBox<D, T> {
    pub fn from(first: Point<D, T>, second: Point<D, T>) -> BoundingBox<D, T>
    where
        T: PartialOrd + Copy,
    {
        let mut min_coords: [T; D] = first.coords;
        let mut max_coords: [T; D] = second.coords;

        for ix in 0..D {
            if second.coords[ix] < min_coords[ix] {
                min_coords[ix] = second.coords[ix];
            }
            if first.coords[ix] > max_coords[ix] {
                max_coords[ix] = first.coords[ix];
            }
        }

        BoundingBox { min: Point { coords: min_coords }, max: Point { coords: max_coords } }
    }

    pub fn expand(&mut self, amount: Vector<D, T>)
    where
        T: Copy + Add<Output = T> + Sub<Output = T>,
    {
        self.min = self.min - amount;
        self.max = self.max + amount;
    }

    pub fn min_point(&self) -> Point<D, T>
    where
        T: Copy,
    {
        self.min
    }

    pub fn max_point(&self) -> Point<D, T>
    where
        T: Copy,
    {
        self.max
    }
}

impl<const D: usize, T: Sub<Output = T> + Copy> BoundingBox<D, T> {
    pub fn total_size(&self) -> Vector<D, T> { self.max - self.min }
}

impl<const D: usize, T> BoundingBox<D, T>
where
    T: PartialOrd,
{
    pub fn contains_inclusive(&self, point: &Point<D, T>) -> bool {
        for ix in 0..D {
            if point.coords[ix] < self.min.coords[ix] || point.coords[ix] > self.max.coords[ix] {
                return false;
            }
        }
        true
    }
}

pub trait FindBoundingBox<const D: usize, T> {
    fn enclosing_rect(self) -> Option<BoundingBox<D, T>>;
}

impl<const D: usize, T: PartialOrd, I> FindBoundingBox<D, T> for I
where
    T: PartialOrd + Copy,
    I: Iterator<Item = Point<D, T>>,
{
    fn enclosing_rect(mut self) -> Option<BoundingBox<D, T>> {
        let mut min = self.next()?;
        let mut max = min;

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

        Some(BoundingBox { min, max })
    }
}

impl<'a, const D: usize, T> Parse<'a> for Point<D, T>
where
    T: Default + Copy + FromStr,
    <T as FromStr>::Err: Display,
{
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(Point { coords: parse_coords(s).map_err(ParseError::new)? })
    }
}

impl<'a, const D: usize, T> Parse<'a> for Vector<D, T>
where
    T: Default + Copy + FromStr,
    <T as FromStr>::Err: Display,
{
    fn from_str(s: &'a str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(Vector { coords: parse_coords(s).map_err(ParseError::new)? })
    }
}

pub struct PointIterator<const D: usize, T> {
    point: Point<D, T>,
    direction: Vector<D, T>,
}

impl<const D: usize, T> PointIterator<D, T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn new(start: Point<D, T>, direction: Vector<D, T>) -> PointIterator<D, T> {
        PointIterator { point: start - direction, direction }
    }
}

impl<const D: usize, T> Iterator for PointIterator<D, T>
where
    T: Copy + Add<Output = T>,
{
    type Item = Point<D, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.point = self.point + self.direction;
        Some(self.point)
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
