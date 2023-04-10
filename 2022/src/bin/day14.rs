use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::grid::Grid;
use advent_lib::iter_utils::ZipWithNextTrait;
use advent_lib::vec2::{LineSegment, Vec2};

struct Day {
    grid: Grid<Place>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let lines: Vec<LineSegment<i32>> = iter
            .into_iter()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|str| str.parse().unwrap())
                    .zip_with_next()
                    .map(|pair| pair.into())
                    .collect::<Vec<_>>()
            })
            .collect();

        let max_height = lines.iter().map(|line| line.end.y).max().unwrap() + 2;
        let mut grid = Grid::new_empty((500 - max_height)..=(500 + max_height), 0..=max_height);

        for line in lines {
            for x in line.x_range() {
                for y in line.y_range() {
                    let place = grid.get_mut(x, y).unwrap();
                    *place = Place::LINE;
                }
            }
        }

        Day { grid }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn calculate_part1(&self) -> Self::Output { SandDroppingGrid::new(&self.grid).count() }

    fn calculate_part2(&self) -> Self::Output {
        let mut grid = SandDroppingGrid::new(&self.grid);
        let y = *grid.grid.y_range().end();
        for x in grid.grid.x_range() {
            let place = grid.grid.get_mut(x, y).unwrap();
            *place = Place::LINE;
        }
        grid.count()
    }
}

struct SandDroppingGrid {
    grid: Grid<Place>,
    drop_point: Vec2<i32>,
}

impl SandDroppingGrid {
    fn new(grid: &Grid<Place>) -> SandDroppingGrid {
        SandDroppingGrid { grid: grid.clone(), drop_point: Vec2 { x: 500, y: 0 } }
    }
}
impl Iterator for SandDroppingGrid {
    type Item = Vec2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.drop_point.x;
        let mut y = self.drop_point.y;
        if self.grid.get(x, y) != Some(&Place::EMPTY) {
            return None;
        }

        loop {
            if self.grid.get(x, y + 1) == None {
                return None; // Falling any lower will drop us off the grid
            } else if self.grid.get(x, y + 1) == Some(&Place::EMPTY) {
                y += 1;
            } else if self.grid.get(x - 1, y + 1) == Some(&Place::EMPTY) {
                x -= 1;
                y += 1;
            } else if self.grid.get(x + 1, y + 1) == Some(&Place::EMPTY) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        if let Some(place) = self.grid.get_mut(x, y) {
            *place = Place::SAND;
            Some(Vec2 { x, y })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Place {
    #[default]
    EMPTY,
    SAND,
    LINE,
}

impl From<Place> for char {
    fn from(value: Place) -> Self {
        match value {
            Place::EMPTY => '.',
            Place::SAND => 'o',
            Place::LINE => '#',
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 14, example => 24, 93 );
    day_test!( 14 => 843, 27625 );
}
