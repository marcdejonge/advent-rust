use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut, RangeInclusive};

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
        Grid::new(lines.map(|line| line.bytes().map(T::from).collect()).collect())
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

    pub fn new(items: Vec<Vec<T>>) -> Grid<T>
    where
        T: Clone,
    {
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
        if !self.x_indices.contains(&location.coords[0])
            || !self.y_indices.contains(&location.coords[1])
        {
            None
        } else {
            Some(
                (location.coords[0] - self.x_indices.start()) as usize
                    + (location.coords[1] - self.y_indices.start()) as usize * self.width,
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
        if line.start.coords[0] == line.end.coords[0] {
            let x = line.start.coords[0];
            for y in line.min_y()..=line.max_y() {
                if let Some(place) = self.get_mut(point2(x, y)) {
                    *place = value
                }
            }
        } else if line.start.coords[1] == line.end.coords[1] {
            let y = line.start.coords[1];
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
}

impl<T: Copy + Into<u8>> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x = ")?;
        self.x_indices.fmt(f)?;
        f.write_str(", y = ")?;
        self.y_indices.fmt(f)?;
        f.write_str("\n\n")?;

        for y in self.y_indices.clone() {
            for x in self.x_indices.clone() {
                let item = self.get(point2(x, y)).unwrap();
                f.write_char((*item).into().into())?;
            }
            f.write_char('\n')?;
        }

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
