use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Debug for Point {
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
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn manhattan(&self, other: Point) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
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
