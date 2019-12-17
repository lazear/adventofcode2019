use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

/// A point on an XY plane where the top-left position has coordinate of (0,0)
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// A point on an infinite XY plane, where the origin has coordinates of (0,0)
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Rotation {
    Left,
    Right,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    grid: Vec<T>,
}

pub struct PointIter {
    len: usize,
    cols: usize,
    pos: usize,
}

impl Iterator for PointIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        }
        let y = self.pos / self.cols;
        let x = self.pos % self.cols;
        self.pos += 1;
        Some(Point { x, y })
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, by: Point) -> &Self::Output {
        &self.grid[self.cols * by.y + by.x]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, by: Point) -> &mut Self::Output {
        &mut self.grid[self.cols * by.y + by.x]
    }
}

impl<T> Grid<T> {
    pub fn new(cols: usize, rows: usize, grid: Vec<T>) -> Grid<T> {
        Grid { cols, rows, grid }
    }

    pub fn iter_points(&self) -> PointIter {
        PointIter {
            len: self.cols * self.rows,
            cols: self.cols,
            pos: 0,
        }
    }

    pub fn iter(&self) -> std::iter::Zip<PointIter, std::slice::Iter<T>> {
        self.iter_points().zip(self.grid.iter())
    }

    pub fn in_bounds(&self, pt: Point) -> bool {
        pt.x < self.cols && pt.y < self.rows
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn manhattan(&self, other: Point) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    pub fn move_one(self, dir: Direction) -> Point {
        match dir {
            Direction::Up => Point {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Point {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .chunks_exact(self.cols)
                .map(|row| row
                    .iter()
                    .map(|sp| sp.to_string())
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    /// Convert a set of coordinates on an infinite plane centerd on 0,0 to
    /// a finite set of [`Points`] where the top-left-most [`Coord`] is changed
    /// to (0,0), and all other points are modified to have the same relative
    /// position
    pub fn to_grid<T: Default + Clone>(data: HashMap<Coord, T>) -> Grid<T> {
        assert!(data.len() > 0);
        let min_x = data.keys().map(|c| c.x).min().unwrap();
        let min_y = data.keys().map(|c| c.y).min().unwrap();
        let max_x = data.keys().map(|c| c.x).max().expect("max_x fail");
        let max_y = data.keys().map(|c| c.y).max().expect("max_y fail");

        let cols = usize::try_from(max_x - min_x).expect("cols failed") + 1;
        let rows = usize::try_from(max_y - min_y).expect("rows failed") + 1;

        let g = std::iter::repeat(T::default())
            .take(cols * rows)
            .collect::<Vec<T>>();

        let mut grid = Grid::new(cols, rows, g);

        for (coord, val) in data {
            let pt = Point::new(
                (coord.x + min_x.abs()) as usize,
                (coord.y + min_y.abs()) as usize,
            );
            grid[pt] = val;
        }
        grid
    }

    pub fn move_one(self, dir: Direction) -> Coord {
        match dir {
            Direction::Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Grid<T> {
        Grid {
            cols: self.cols,
            rows: self.rows,
            grid: self.grid.clone(),
        }
    }
}

impl Direction {
    pub fn rotate(self, rotation: Rotation) -> Direction {
        match rotation {
            Rotation::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Rotation::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        }
    }
}

#[test]
fn grid_indexing() {
    let grid: Grid<i32> = Grid {
        cols: 3,
        rows: 3,
        grid: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };
    assert_eq!(grid[Point::new(0, 0)], 0i32);
    assert_eq!(grid[Point::new(1, 1)], 4i32);
    assert_eq!(grid[Point::new(2, 2)], 8i32);
}

#[test]
fn grid_iter() {
    let grid: Grid<i32> = Grid {
        cols: 3,
        rows: 3,
        grid: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    let iter = grid.iter_points();
    assert_eq!(
        iter.take(5).collect::<Vec<_>>(),
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(0, 1),
            Point::new(1, 1)
        ]
    );
}
