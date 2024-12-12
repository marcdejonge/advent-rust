use crate::direction::ALL_DIRECTIONS;
use crate::geometry::{point2, vector2, Point, PointIterator, Vector};
use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut, Range};

#[derive(Clone, Hash)]
pub struct Grid<T> {
    items: Vec<T>,
    size: Size,
}

pub type Location = Point<2, i32>;

pub type Size = Vector<2, i32>;

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

    fn index_from_location_infinite(&self, location: Location) -> usize {
        let x = location.x().rem_euclid(self.width()) as usize;
        let y = location.y().rem_euclid(self.height()) as usize;
        y * self.width() as usize + x
    }

    pub fn get_infinite(&self, location: Location) -> &T {
        self.items.get(self.index_from_location_infinite(location)).unwrap()
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

    pub fn locations(&self) -> impl Iterator<Item = Location> {
        let ys = self.y_range();
        let xs = self.x_range();

        ys.flat_map(move |y| xs.clone().map(move |x| point2(x, y)))
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

    pub fn map_entries<U, F>(&self, function: F) -> Grid<U>
    where
        F: Fn(Location, &T) -> U,
    {
        let mut items = Vec::with_capacity(self.items.len());
        self.items
            .iter()
            .enumerate()
            .map(|(ix, value)| function((self.size, ix).into(), value))
            .for_each(|result| items.push(result));
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

    /// # Safety
    ///
    /// This method does not do any boundary checks, so only use this if you already know that
    /// x and y are within boundary (e.g. coming directly from the x_range and y_range).
    pub unsafe fn get_unchecked(&self, x: i32, y: i32) -> &T {
        self.items.get_unchecked((x + y * self.width()) as usize)
    }

    /// # Safety
    ///
    /// This method does not do any boundary checks, so only use this if you already know that
    /// x and y are within boundary (e.g. coming directly from the x_range and y_range).
    pub unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &mut T {
        let width = self.width();
        self.items.get_unchecked_mut((x + y * width) as usize)
    }

    pub fn north_line(&self, x: i32) -> LineIterator<T> {
        LineIterator::North { grid: self, x, y: self.height() - 1 }
    }

    pub fn east_line(&self, y: i32) -> LineIterator<T> {
        LineIterator::East { grid: self, x: 0, y }
    }

    pub fn south_line(&self, x: i32) -> LineIterator<T> {
        LineIterator::South { grid: self, x, y: 0 }
    }

    pub fn west_line(&self, y: i32) -> LineIterator<T> {
        LineIterator::West { grid: self, x: self.width() - 1, y }
    }

    pub fn north_lines(&self) -> LinesIterator<T> { LinesIterator::North { grid: self, x: 0 } }

    pub fn east_lines(&self) -> LinesIterator<T> { LinesIterator::East { grid: self, y: 0 } }

    pub fn south_lines(&self) -> LinesIterator<T> { LinesIterator::South { grid: self, x: 0 } }

    pub fn west_lines(&self) -> LinesIterator<T> { LinesIterator::West { grid: self, y: 0 } }

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

    pub fn fill(&mut self, start: Location, value: T) -> usize
    where
        T: PartialEq + Clone,
    {
        let accept_cell = if let Some(cell) = self.get(start) {
            cell.clone()
        } else {
            return 0; // Not in range, just return
        };

        let mut stack = Vec::with_capacity(self.height() as usize);
        stack.push(start);
        let mut count_cells = 0usize;

        while !stack.is_empty() {
            let [x, y] = stack.pop().unwrap().coords;
            let mut lx = x;
            while lx > 0 {
                // Safety: we know that (x,y) is in bounds and we check lx explicitly
                let fill_cell = unsafe { self.get_unchecked_mut(lx - 1, y) };
                if *fill_cell == accept_cell {
                    *fill_cell = value.clone();
                    count_cells += 1;
                    lx -= 1;
                } else {
                    break;
                }
            }

            let mut rx = x;
            while rx < self.width() {
                // Safety: we know that (x,y) is in bounds and we check rx explicitly
                let fill_cell = unsafe { self.get_unchecked_mut(rx, y) };
                if *fill_cell == accept_cell {
                    *fill_cell = value.clone();
                    count_cells += 1;
                    rx += 1;
                } else {
                    break;
                }
            }

            let mut scan_next_fill = |y| {
                let mut span_added = false;
                for x in lx..rx {
                    let check_cell = unsafe { self.get_unchecked(x, y) };
                    if *check_cell != accept_cell {
                        span_added = false;
                    } else if !span_added {
                        stack.push(point2(x, y));
                        span_added = true;
                    }
                }
            };

            if y > 0 {
                scan_next_fill(y - 1);
            }
            if y < self.height() - 1 {
                scan_next_fill(y + 1);
            }
        }

        count_cells
    }

    pub fn draw_with_overlay<'a, I>(&self, locations: I, c: char)
    where
        I: IntoIterator<Item = &'a Location>,
        T: Into<char> + Copy,
    {
        let mut char_grid: Grid<char> = self.map(|b| (*b).into());
        for loc in locations {
            if let Some(cell) = char_grid.get_mut(*loc) {
                *cell = c;
            }
        }

        println!("{char_grid:?}");
    }

    pub fn detect_regions(&self) -> Vec<Vec<Location>>
    where
        T: Eq + Clone,
    {
        let mut regions = Vec::new();
        let mut visited = vec![false; self.items.len()];
        for start in self.locations() {
            let start_ix = self.index_from_location(start).unwrap();
            let current_value = self.items.get(start_ix);
            if visited[start_ix] {
                continue;
            }

            let mut region = Vec::new();
            let mut stack = vec![start];
            while let Some(location) = stack.pop() {
                let location_ix = self.index_from_location(location).unwrap();
                if visited[location_ix] {
                    continue;
                }
                visited[location_ix] = true;
                region.push(location);
                for d in ALL_DIRECTIONS {
                    let neighbour = location + d;
                    if self.get(neighbour) == current_value {
                        stack.push(neighbour);
                    }
                }
            }
            regions.push(region);
        }

        regions
    }
}

impl<T: Copy + Into<char>> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Grid(")?;
        f.write_str(&format!("{}x{}", self.width(), self.height()))?;
        f.write_char(')')?;
        f.write_char('\n')?;

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

pub enum LineIterator<'a, T> {
    North { grid: &'a Grid<T>, x: i32, y: i32 },
    East { grid: &'a Grid<T>, x: i32, y: i32 },
    South { grid: &'a Grid<T>, x: i32, y: i32 },
    West { grid: &'a Grid<T>, x: i32, y: i32 },
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = (Location, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LineIterator::North { grid, x, y } => {
                if *y >= 0 {
                    let curr_y = *y;
                    *y -= 1;
                    Some((point2(*x, curr_y), unsafe {
                        grid.get_unchecked(*x, curr_y)
                    }))
                } else {
                    None
                }
            }
            LineIterator::East { grid, x, y } => {
                if *x < grid.width() {
                    let curr_x = *x;
                    *x += 1;
                    Some((point2(curr_x, *y), unsafe {
                        grid.get_unchecked(curr_x, *y)
                    }))
                } else {
                    None
                }
            }
            LineIterator::South { grid, x, y } => {
                if *y < grid.height() {
                    let curr_y = *y;
                    *y += 1;
                    Some((point2(*x, curr_y), unsafe {
                        grid.get_unchecked(*x, curr_y)
                    }))
                } else {
                    None
                }
            }
            LineIterator::West { grid, x, y } => {
                if *x >= 0 {
                    let curr_x = *x;
                    *x -= 1;
                    Some((point2(curr_x, *y), unsafe {
                        grid.get_unchecked(curr_x, *y)
                    }))
                } else {
                    None
                }
            }
        }
    }
}

pub enum LinesIterator<'a, T> {
    North { grid: &'a Grid<T>, x: i32 },
    East { grid: &'a Grid<T>, y: i32 },
    South { grid: &'a Grid<T>, x: i32 },
    West { grid: &'a Grid<T>, y: i32 },
}

impl<'a, T> Iterator for LinesIterator<'a, T> {
    type Item = LineIterator<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LinesIterator::North { grid, x } => {
                if *x < grid.width() {
                    let curr_x = *x;
                    *x += 1;
                    Some(LineIterator::North { grid, x: curr_x, y: grid.height() - 1 })
                } else {
                    None
                }
            }
            LinesIterator::East { grid, y } => {
                if *y < grid.height() {
                    let curr_y = *y;
                    *y += 1;
                    Some(LineIterator::East { grid, x: 0, y: curr_y })
                } else {
                    None
                }
            }
            LinesIterator::South { grid, x } => {
                if *x < grid.width() {
                    let curr_x = *x;
                    *x += 1;
                    Some(LineIterator::South { grid, x: curr_x, y: 0 })
                } else {
                    None
                }
            }
            LinesIterator::West { grid, y } => {
                if *y < grid.height() {
                    let curr_y = *y;
                    *y += 1;
                    Some(LineIterator::West { grid, x: grid.width() - 1, y: curr_y })
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point2;

    use super::Grid;

    fn generate_test_grid() -> Grid<u8> {
        Grid::from("123\n456\n789".lines().map(str::to_owned)).map(|b| b - b'0')
    }

    #[test]
    fn test_north_iterators() {
        let grid = generate_test_grid();
        let cells = grid.north_lines().flat_map(|line| line.map(|(_, c)| *c)).collect::<Vec<_>>();
        assert_eq!([7, 4, 1, 8, 5, 2, 9, 6, 3], cells.as_slice())
    }
    #[test]
    fn test_east_iterators() {
        let grid = generate_test_grid();
        let cells = grid.east_lines().flat_map(|line| line.map(|(_, c)| *c)).collect::<Vec<_>>();
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], cells.as_slice())
    }
    #[test]
    fn test_south_iterators() {
        let grid = generate_test_grid();
        let cells = grid.south_lines().flat_map(|line| line.map(|(_, c)| *c)).collect::<Vec<_>>();
        assert_eq!([1, 4, 7, 2, 5, 8, 3, 6, 9], cells.as_slice())
    }
    #[test]
    fn test_west_iterators() {
        let grid = generate_test_grid();
        let cells = grid.west_lines().flat_map(|line| line.map(|(_, c)| *c)).collect::<Vec<_>>();
        assert_eq!([3, 2, 1, 6, 5, 4, 9, 8, 7], cells.as_slice())
    }

    #[test]
    fn test_fill_around_block() {
        let mut grid = Grid::<u8>::new_empty(3, 3);
        grid[point2(1, 1)] = b'X';
        assert_eq!(8, grid.fill(point2(0, 0), b'O'));
        assert_eq!(
            vec![b'O', b'O', b'O', b'O', b'X', b'O', b'O', b'O', b'O'],
            grid.items
        );
    }

    #[test]
    fn test_fill_inside() {
        let mut grid = Grid::<u8>::new_empty(7, 2);
        grid[point2(1, 0)] = b'X';
        grid[point2(3, 0)] = b'X';
        grid[point2(5, 0)] = b'X';
        assert_eq!(11, grid.fill(point2(0, 0), b'O'));
        assert_eq!(
            vec![
                b'O', b'X', b'O', b'X', b'O', b'X', b'O', b'O', b'O', b'O', b'O', b'O', b'O', b'O'
            ],
            grid.items
        );
    }
}
