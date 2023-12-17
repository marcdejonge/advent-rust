use crate::direction::Direction::{East, North, South, West};
use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut, Range};

use crate::geometry::{point2, vector2, Point, PointIterator, Vector};

#[derive(Clone, Hash)]
pub struct Grid<T> {
    items: Vec<T>,
    size: Size,
}

type Location = Point<2, i32>;

type Size = Vector<2, i32>;

impl From<(Size, usize)> for Location {
    fn from(value: (Size, usize)) -> Self {
        let index = value.1 as i32;
        let x = index % value.0.x();
        let y = index / value.0.x();
        point2(x, y)
    }
}

impl<I, T> From<I> for Grid<T>
where
    T: From<u8> + Clone,
    I: Iterator<Item = String>,
{
    fn from(lines: I) -> Self {
        let items: Vec<Vec<T>> = lines.map(|line| line.bytes().map(T::from).collect()).collect();

        let height = items.len();
        let width = items[0].len();
        if items.iter().any(|line| line.len() != width) {
            panic!("Not all input lines have the same length");
        }
        Grid {
            items: items.iter().flatten().cloned().collect(),
            size: vector2(width as i32, height as i32),
        }
    }
}

impl<T> Grid<T> {
    pub fn new_empty(width: i32, height: i32) -> Grid<T>
    where
        T: Default + Clone,
    {
        Grid::new_default(T::default(), width, height)
    }

    pub fn new_default(value: T, width: i32, height: i32) -> Grid<T>
    where
        T: Clone,
    {
        if width < 0 {
            panic!("Width cannot be negative")
        } else if height < 0 {
            panic!("Height cannot be negative")
        }
        Grid {
            items: vec![value.clone(); (width * height) as usize],
            size: vector2(width, height),
        }
    }

    pub fn height(&self) -> i32 { self.size.y() }

    pub fn width(&self) -> i32 { self.size.x() }

    pub fn len(&self) -> usize { self.items.len() }

    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    pub fn x_range(&self) -> Range<i32> { 0..self.size.x() }

    pub fn y_range(&self) -> Range<i32> { 0..self.size.y() }

    pub fn is_valid_location(&self, location: &Location) -> bool {
        self.x_range().contains(&location.x()) && self.y_range().contains(&location.y())
    }

    fn index_from_location(&self, location: Location) -> Option<usize> {
        if self.is_valid_location(&location) {
            Some((location.x() + location.y() * self.width()) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, location: Location) -> Option<&T> {
        let ix = self.index_from_location(location)?;
        self.items.get(ix)
    }

    pub fn get_mut(&mut self, location: Location) -> Option<&mut T> {
        let ix = self.index_from_location(location)?;
        self.items.get_mut(ix)
    }

    pub fn swap(&mut self, first: Location, second: Location) -> Result<(), &str> {
        if first == second {
            return Ok(()); // Nothing to swap
        }

        let first_ix = self.index_from_location(first).ok_or("Could not find first index")?;
        let second_ix = self.index_from_location(second).ok_or("Could not find second index")?;

        self.items.swap(first_ix, second_ix);

        Ok(())
    }

    pub fn entries(&self) -> impl Iterator<Item = (Location, &T)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, value)| ((self.size, index).into(), value))
    }

    pub fn entries_mut(&mut self) -> impl Iterator<Item = (Location, &mut T)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, value)| ((self.size, index).into(), value))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> { self.items.iter() }

    pub fn map<U, F>(&self, function: F) -> Grid<U>
    where
        F: FnMut(&T) -> U,
    {
        let mut items = Vec::with_capacity(self.items.len());
        self.items.iter().map(function).for_each(|result| items.push(result));
        Grid { items, size: self.size }
    }

    pub fn find<F>(&self, mut predicate: F) -> Option<Location>
    where
        T: PartialEq,
        F: FnMut(&T) -> bool,
    {
        let (index, _) = self.items.iter().enumerate().find(|(_, item)| predicate(*item))?;
        Some((self.size, index).into())
    }

    pub fn north_line(&self, x: i32) -> LineIterator<T> {
        LineIterator { grid: self, location: point2(x, self.height()), direction: North.as_vec() }
    }

    pub fn east_line(&self, y: i32) -> LineIterator<T> {
        LineIterator { grid: self, location: point2(-1, y), direction: East.as_vec() }
    }

    pub fn south_line(&self, x: i32) -> LineIterator<T> {
        LineIterator { grid: self, location: point2(x, -1), direction: South.as_vec() }
    }

    pub fn west_line(&self, y: i32) -> LineIterator<T> {
        LineIterator { grid: self, location: point2(self.width(), y), direction: West.as_vec() }
    }

    pub fn north_lines(&self) -> Vec<LineIterator<T>> {
        let mut result = Vec::with_capacity(self.width() as usize);
        self.x_range().for_each(|x| result.push(self.north_line(x)));
        result
    }

    pub fn east_lines(&self) -> Vec<LineIterator<T>> {
        let mut result = Vec::with_capacity(self.height() as usize);
        self.y_range().for_each(|y| result.push(self.east_line(y)));
        result
    }

    pub fn south_lines(&self) -> Vec<LineIterator<T>> {
        let mut result = Vec::with_capacity(self.width() as usize);
        self.x_range().for_each(|x| result.push(self.south_line(x)));
        result
    }

    pub fn west_lines(&self) -> Vec<LineIterator<T>> {
        let mut result = Vec::with_capacity(self.height() as usize);
        self.y_range().for_each(|y| result.push(self.west_line(y)));
        result
    }

    pub fn mut_line<F>(&mut self, start: Location, direction: Vector<2, i32>, function: F)
    where
        F: Fn(&mut T),
    {
        for loc in PointIterator::new(start, direction) {
            if let Some(value) = self.get_mut(loc) {
                function(value);
            } else {
                break;
            }
        }
    }

    pub fn sub_grid(&self, from_x_range: Range<i32>, from_y_range: Range<i32>) -> Grid<T>
    where
        T: Default + Clone,
    {
        let mut grid = Grid::new_empty(
            from_x_range.end - from_x_range.start,
            from_y_range.end - from_y_range.start,
        );
        for y in from_y_range.clone() {
            for x in from_x_range.clone() {
                if let Some(cell) = self.get(point2(x, y)) {
                    let target_x = x - from_x_range.start;
                    let target_y = y - from_y_range.start;

                    if let Some(target) = grid.get_mut(point2(target_x, target_y)) {
                        *target = cell.clone();
                    }
                }
            }
        }

        grid
    }
}

impl<T: Copy + Into<char>> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('┌')?;
        for _ in self.x_range() {
            f.write_char('─')?;
        }
        f.write_char('┐')?;
        f.write_char('\n')?;

        for y in self.y_range() {
            f.write_char('│')?;
            for x in self.x_range() {
                let item = self.get(point2(x, y)).unwrap();
                f.write_char((*item).into())?;
            }
            f.write_char('│')?;
            f.write_char('\n')?;
        }

        f.write_char('└')?;
        for _ in self.x_range() {
            f.write_char('─')?;
        }
        f.write_char('┘')?;
        f.write_char('\n')?;

        Ok(())
    }
}

impl<T> Index<Location> for Grid<T> {
    type Output = T;

    fn index(&self, index: Location) -> &Self::Output { self.get(index).unwrap() }
}

impl<T> IndexMut<Location> for Grid<T> {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

pub struct LineIterator<'a, T> {
    grid: &'a Grid<T>,
    location: Location,
    direction: Vector<2, i32>,
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = (Location, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.location = self.location + self.direction;
        Some((self.location, self.grid.get(self.location)?))
    }
}
