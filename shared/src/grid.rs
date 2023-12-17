use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut, Range, RangeInclusive};

use crate::direction::Direction;
use crossbeam::scope;

use crate::geometry::{point2, Point, PointIterator, Vector};
use crate::lines::LineSegment;

#[derive(Clone, Hash)]
pub struct Grid<T> {
    items: Vec<T>,
    x_indices: RangeInclusive<i32>,
    y_indices: RangeInclusive<i32>,
    width: usize,
    height: usize,
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
            height,
            width,
            x_indices: (0..=(width - 1) as i32),
            y_indices: (0..=(height - 1) as i32),
        }
    }
}

impl<T> Grid<T> {
    pub fn new_empty(x_indices: RangeInclusive<i32>, y_indices: RangeInclusive<i32>) -> Grid<T>
    where
        T: Default + Clone,
    {
        Grid::new_default(T::default(), x_indices, y_indices)
    }

    pub fn new_default(
        value: T,
        x_indices: RangeInclusive<i32>,
        y_indices: RangeInclusive<i32>,
    ) -> Grid<T>
    where
        T: Clone,
    {
        let width = (x_indices.end() - x_indices.start() + 1) as usize;
        let height = (y_indices.end() - y_indices.start() + 1) as usize;
        Grid { items: vec![value.clone(); width * height], x_indices, y_indices, width, height }
    }

    #[inline]
    pub fn for_all_lines<C, LC>(
        &self,
        result: &mut C,
        line_context: &LC,
        each_cell: fn(&mut C, &mut LC, &T, usize),
        combine: fn(&mut C, C),
    ) where
        C: Clone + Sync + Send,
        LC: Clone + Sync,
        T: Sync + Send,
    {
        let thread_results = scope(|s| {
            [
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for y in 0..self.height {
                        let mut lc = line_context.clone();
                        for index in (y * self.width)..((y + 1) * self.width) {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for y in 0..self.height {
                        let mut lc = line_context.clone();
                        for index in ((y * self.width)..((y + 1) * self.width)).rev() {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for x in 0..self.width {
                        let mut lc = line_context.clone();
                        for index in (x..(self.height * self.width)).step_by(self.width) {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for x in 0..self.width {
                        let mut lc = line_context.clone();
                        for index in (x..(self.height * self.width)).step_by(self.width).rev() {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
            ]
            .map(|thread| thread.join().unwrap())
        })
        .unwrap();

        for thread_result in thread_results {
            combine(result, thread_result)
        }
    }

    pub fn len(&self) -> usize { self.items.len() }

    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    pub fn calc_index(&self, location: Point<2, i32>) -> Option<usize> {
        if !self.x_indices.contains(&location.x()) || !self.y_indices.contains(&location.y()) {
            None
        } else {
            Some(
                (location.x() - self.x_indices.start()) as usize
                    + (location.y() - self.y_indices.start()) as usize * self.width,
            )
        }
    }

    pub fn get(&self, location: Point<2, i32>) -> Option<&T> {
        self.items.get(self.calc_index(location)?)
    }

    pub fn get_mut(&mut self, location: Point<2, i32>) -> Option<&mut T> {
        let ix = self.calc_index(location)?;
        self.items.get_mut(ix)
    }

    pub fn x_range(&self) -> RangeInclusive<i32> { self.x_indices.clone() }

    pub fn y_range(&self) -> RangeInclusive<i32> { self.y_indices.clone() }

    pub fn contains(&self, location: &Point<2, i32>) -> bool {
        self.x_range().contains(&location.x()) && self.y_range().contains(&location.y())
    }

    pub fn height(&self) -> usize { self.height }

    pub fn width(&self) -> usize { self.width }

    pub fn swap(&mut self, first: Point<2, i32>, second: Point<2, i32>) -> Result<(), &str> {
        if first == second {
            return Ok(()); // Nothing to swap
        }

        let first_ix = self.calc_index(first).ok_or("Could not find first index")?;
        let second_ix = self.calc_index(second).ok_or("Could not find second index")?;

        self.items.swap(first_ix, second_ix);

        Ok(())
    }

    pub fn draw_line(&mut self, line: LineSegment<i32>, value: T)
    where
        T: Copy,
    {
        if line.start.x() == line.end.x() {
            let x = line.start.x();
            for y in line.min_y()..=line.max_y() {
                if let Some(place) = self.get_mut(point2(x, y)) {
                    *place = value
                }
            }
        } else if line.start.y() == line.end.y() {
            let y = line.start.y();
            for x in line.min_x()..=line.max_x() {
                if let Some(place) = self.get_mut(point2(x, y)) {
                    *place = value
                }
            }
        } else {
            unimplemented!("Non-straight lines cannot be drawn to a Grid yet")
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = (Point<2, i32>, &T)> {
        self.items.iter().enumerate().map(|(index, value)| {
            let y = index / self.width;
            let x = index - y * self.width;
            (
                point2(
                    (x as i32) + self.x_indices.start(),
                    (y as i32) + self.y_indices.start(),
                ),
                value,
            )
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &T> { self.items.iter() }

    pub fn iter_hor_line(&self, y: i32) -> impl Iterator<Item = &T> {
        self.iter_line(point2(0, y), Direction::East.as_vec())
    }

    pub fn iter_ver_line(&self, x: i32) -> impl Iterator<Item = &T> {
        self.iter_line(point2(x, 0), Direction::South.as_vec())
    }

    pub fn iter_line(
        &self,
        start: Point<2, i32>,
        direction: Vector<2, i32>,
    ) -> impl Iterator<Item = &T> {
        PointIterator::new(start, direction)
            .map(|loc| self.get(loc))
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
    }

    pub fn mut_line<F>(&mut self, start: Point<2, i32>, direction: Vector<2, i32>, function: F)
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

    pub fn map<U, F>(&self, function: F) -> Grid<U>
    where
        F: FnMut(&T) -> U,
    {
        let mut items = Vec::with_capacity(self.items.len());
        self.items.iter().map(function).for_each(|result| items.push(result));

        Grid {
            items,
            x_indices: self.x_indices.clone(),
            y_indices: self.y_indices.clone(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn find<F>(&self, mut predicate: F) -> Option<Point<2, i32>>
    where
        T: PartialEq,
        F: FnMut(&T) -> bool,
    {
        let (index, _) = self.items.iter().enumerate().find(|(_, item)| predicate(*item))?;

        let y = index / self.width;
        let x = index % self.width;

        Some(point2(
            (x as i32) + self.x_indices.start(),
            (y as i32) + self.y_indices.start(),
        ))
    }

    pub fn sub_grid(&self, from_x_range: Range<i32>, from_y_range: Range<i32>) -> Grid<T>
    where
        T: Default + Clone,
    {
        let mut grid = Grid::new_empty(
            0..=(from_x_range.end - from_x_range.start - 1),
            0..=(from_y_range.end - from_y_range.start - 1),
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
        f.write_str("x = ")?;
        self.x_indices.fmt(f)?;
        f.write_str(", y = ")?;
        self.y_indices.fmt(f)?;
        f.write_str("\n\n")?;

        f.write_char('┌')?;
        for _ in self.x_indices.clone() {
            f.write_char('─')?;
        }
        f.write_char('┐')?;
        f.write_char('\n')?;

        for y in self.y_indices.clone() {
            f.write_char('│')?;
            for x in self.x_indices.clone() {
                let item = self.get(point2(x, y)).unwrap();
                f.write_char((*item).into())?;
            }
            f.write_char('│')?;
            f.write_char('\n')?;
        }

        f.write_char('└')?;
        for _ in self.x_indices.clone() {
            f.write_char('─')?;
        }
        f.write_char('┘')?;
        f.write_char('\n')?;

        Ok(())
    }
}

impl<T> Index<Point<2, i32>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point<2, i32>) -> &Self::Output { self.get(index).unwrap() }
}

impl<T> IndexMut<Point<2, i32>> for Grid<T> {
    fn index_mut(&mut self, index: Point<2, i32>) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
