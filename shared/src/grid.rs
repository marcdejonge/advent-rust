use crate::direction::Direction;
use crate::geometry::{Point, PointIterator, Vector, point2, vector2};
use bit_vec::BitVec;
use image::{Rgba, RgbaImage};
use nom::Err::Error;
use nom::character::complete::{line_ending, not_line_ending};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::preceded;
use nom::{AsBytes, AsChar, Compare, IResult, Input, Parser};
use nom_parse_trait::ParseFrom;
use std::fmt::{Debug, Formatter, Write};
use std::ops::{Add, Index, IndexMut, Range};

#[derive(Clone, Hash, PartialEq)]
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

impl<I, E, T> ParseFrom<I, E> for Grid<T>
where
    T: ParseFrom<I, E>,
    E: ParseError<I>,
    I: AsBytes + Input,
    <I as Input>::Item: AsChar,
    I: Compare<&'static str>,
{
    fn parse(input: I) -> IResult<I, Self, E> {
        let mut line_parser = not_line_ending.and_then(many1(T::parse));
        let (rest, first_line) = line_parser.parse(input.clone())?;
        let width = first_line.len();

        let (rest, mut lines) = many0(preceded(line_ending, line_parser)).parse(rest.clone())?;
        if lines.iter().any(|line| line.len() != width) {
            return Err(Error(E::from_error_kind(rest, ErrorKind::LengthValue)));
        }

        let height = lines.len() + 1;
        let mut items = first_line;
        lines.iter_mut().for_each(|line| items.append(line));

        Ok((
            rest,
            Grid { items, size: vector2(width as i32, height as i32) },
        ))
    }
}

/// Parses a Grid with uneven line lengths. This all append all lines with the default T value
/// to be square.
pub fn uneven_grid_parser<I, E, T>(input: I) -> IResult<I, Grid<T>, E>
where
    T: ParseFrom<I, E> + Clone + Default,
    E: ParseError<I>,
    I: AsBytes + Input,
    <I as Input>::Item: AsChar,
    I: Compare<&'static str>,
{
    let (rest, lines) = separated_list1(line_ending, not_line_ending.and_then(many1(T::parse)))
        .parse(input.clone())?;
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let height = lines.len();

    let mut grid = Grid::new_empty(width as i32, height as i32);
    for (y, line) in lines.into_iter().enumerate() {
        for (x, item) in line.into_iter().enumerate() {
            grid[point2(x as i32, y as i32)] = item;
        }
    }

    Ok((rest, grid))
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

    pub fn size(&self) -> Vector<2, i32> { self.size }

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

    pub fn direct_neighbours(&self, location: Location) -> impl Iterator<Item = (Direction, &T)> {
        Direction::ALL
            .into_iter()
            .flat_map(move |dir| self.get(location + dir).map(|p| (dir, p)))
    }

    pub fn cardinal_neighbours(&self, location: Location) -> Option<[&T; 8]>
    where
        T: Default + Copy,
    {
        // Check boundaries, we don't support fetching the neighbours at the edge
        if location.x() <= 0
            || location.x() >= self.width()
            || location.y() <= 0
            || location.y() >= self.height()
        {
            return None;
        }

        let ix = (location.x() + location.y() * self.width()) as usize;
        let y_step = self.width() as usize;

        // We've checked the bounds
        unsafe {
            Some([
                self.items.get_unchecked(ix - y_step),
                self.items.get_unchecked(ix - y_step + 1),
                self.items.get_unchecked(ix + 1),
                self.items.get_unchecked(ix + y_step + 1),
                self.items.get_unchecked(ix + y_step),
                self.items.get_unchecked(ix + y_step - 1),
                self.items.get_unchecked(ix - 1),
                self.items.get_unchecked(ix - y_step - 1),
            ])
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

    pub fn swap(&mut self, first: Location, second: Location) {
        if first == second {
            return; // Nothing to swap
        }

        if let Some(first_ix) = self.index_from_location(first)
            && let Some(second_ix) = self.index_from_location(second)
        {
            self.items.swap(first_ix, second_ix);
        }
    }

    pub fn locations(&self) -> impl Iterator<Item = Location> {
        let ys = self.y_range();
        let xs = self.x_range();

        ys.flat_map(move |y| xs.clone().map(move |x| point2(x, y)))
    }

    pub fn entries(&self) -> impl Iterator<Item = (Location, &T)> {
        Indexed::new(self.items.iter(), self.width())
    }

    pub fn entries_mut(&mut self) -> impl Iterator<Item = (Location, &mut T)> {
        let width = self.width();
        Indexed::new(self.items.iter_mut(), width)
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
        unsafe { self.items.get_unchecked((x + y * self.width()) as usize) }
    }

    /// # Safety
    ///
    /// This method does not do any boundary checks, so only use this if you already know that
    /// x and y are within boundary (e.g. coming directly from the x_range and y_range).
    pub unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &mut T {
        let width = self.width();
        unsafe { self.items.get_unchecked_mut((x + y * width) as usize) }
    }

    pub fn north_line<'a>(&'a self, x: i32) -> LineIterator<'a, T> {
        LineIterator::North { grid: self, x, y: self.height() - 1 }
    }

    pub fn east_line<'a>(&'a self, y: i32) -> LineIterator<'a, T> {
        LineIterator::East { grid: self, x: 0, y }
    }

    pub fn south_line<'a>(&'a self, x: i32) -> LineIterator<'a, T> {
        LineIterator::South { grid: self, x, y: 0 }
    }

    pub fn west_line<'a>(&'a self, y: i32) -> LineIterator<'a, T> {
        LineIterator::West { grid: self, x: self.width() - 1, y }
    }

    pub fn north_lines<'a>(&'a self) -> LinesIterator<'a, T> {
        LinesIterator::North { grid: self, x: 0 }
    }

    pub fn east_lines<'a>(&'a self) -> LinesIterator<'a, T> {
        LinesIterator::East { grid: self, y: 0 }
    }

    pub fn south_lines<'a>(&'a self) -> LinesIterator<'a, T> {
        LinesIterator::South { grid: self, x: 0 }
    }

    pub fn west_lines<'a>(&'a self) -> LinesIterator<'a, T> {
        LinesIterator::West { grid: self, y: 0 }
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

    pub fn draw_with_overlay<'a, I>(&self, locations: I, c: char) -> String
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

        format!("{char_grid:?}")
    }

    pub fn detect_regions(&self) -> Vec<Vec<Location>>
    where
        T: Eq + Clone,
    {
        let mut regions = Vec::new();
        let mut visited = BitVec::from_elem(self.items.len(), false);
        for start in self.locations() {
            let start_ix = self.index_from_location(start).unwrap();
            if visited[start_ix] {
                continue;
            }
            let current_value = self.items[start_ix].clone();

            let mut region = Vec::new();
            let mut stack = vec![start];
            while let Some(location) = stack.pop() {
                let location_ix = self.index_from_location(location).unwrap();
                if visited[location_ix] {
                    continue;
                }
                visited.set(location_ix, true);
                region.push(location);
                for d in Direction::ALL {
                    let neighbour = location + d;
                    if let Some(neighbour_ix) = self.index_from_location(neighbour) {
                        unsafe {
                            if self.items.get_unchecked(neighbour_ix).clone() == current_value
                                && !visited.get_unchecked(neighbour_ix)
                            {
                                stack.push(neighbour);
                            }
                        }
                    }
                }
            }
            regions.push(region);
        }

        regions
    }

    pub fn render_to_image(&self, filename: &str, mapping: impl Fn(&T) -> [u8; 4]) {
        let mut image = RgbaImage::new(self.width() as u32, self.height() as u32);
        self.entries().for_each(|(loc, val)| {
            let pixel = image.get_pixel_mut(loc.x() as u32, loc.y() as u32);
            *pixel = Rgba(mapping(val));
        });
        image
            .save_with_format(filename, image::ImageFormat::Png)
            .expect("Expect saving to not be a problem");
    }

    pub fn search_graph<'a, FS, FH, S>(
        &'a self,
        goal: Location,
        score_step: FS,
        heuristic_score: FH,
    ) -> GridGraph<'a, T, FS, FH>
    where
        FS: Fn(Location, &T, &T) -> Option<S>,
        S: Copy + Default + Eq + Add<S, Output = S> + Ord,
        FH: Fn(Vector<2, i32>) -> S,
    {
        GridGraph::<'a, T, FS, FH> { grid: self, goal, score_step, heuristic_score }
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
    use super::Grid;
    use crate::geometry::point2;
    use nom::error::Error;
    use nom_parse_trait::ParseFromExt;

    fn generate_test_grid() -> Grid<u8> {
        let result: Result<Grid<char>, Error<_>> = Grid::parse_complete("123\n456\n789");
        result.unwrap().map(|&c| c as u8 - b'0')
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

#[derive(Clone, Debug)]
pub struct Indexed<I> {
    iter: I,
    x: i32,
    y: i32,
    width: i32,
}

impl<I> Indexed<I> {
    fn new(iter: I, width: i32) -> Indexed<I> { Indexed { iter, x: 0, y: 0, width } }
}

impl<I> Iterator for Indexed<I>
where
    I: Iterator,
{
    type Item = (Location, <I as Iterator>::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let location = point2(self.x, self.y);
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        Some((location, item))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub struct GridGraph<'a, T, FS, FH> {
    grid: &'a Grid<T>,
    goal: Location,
    score_step: FS,
    heuristic_score: FH,
}

impl<'a, T, S, FS, FH> crate::search::SearchGraph for GridGraph<'a, T, FS, FH>
where
    FS: Fn(Location, &T, &T) -> Option<S>,
    S: Copy + Default + Eq + Add<S, Output = S> + Ord,
    FH: Fn(Vector<2, i32>) -> S,
{
    type Node = Location;
    type Score = S;

    fn neighbours(&self, current_loc: Location) -> impl Iterator<Item = (Location, S)> {
        let current_val = self.grid.get(current_loc).unwrap();
        self.grid.direct_neighbours(current_loc).flat_map(move |(dir, val)| {
            let next_loc = current_loc + dir;
            (self.score_step)(next_loc, current_val, val).map(|score| (next_loc, score))
        })
    }

    fn expected_state_size(&self) -> usize { (self.grid.width() * self.grid.height()) as usize }
}

impl<'a, T, S, FS, FH> crate::search::SearchGraphWithGoal for GridGraph<'a, T, FS, FH>
where
    FS: Fn(Location, &T, &T) -> Option<S>,
    S: Copy + Default + Eq + Add<S, Output = S> + Ord,
    FH: Fn(Vector<2, i32>) -> S,
{
    fn is_goal(&self, curr: Location) -> bool { curr == self.goal }

    fn heuristic(&self, curr: Location) -> Self::Score { (self.heuristic_score)(self.goal - curr) }
}
