use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Default, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

struct PointIter {
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash, Debug)]
enum Slope {
    Vertical(bool),
    Horizontal(bool),
    // Slope * 1000, dy > 0, dx > 0
    Diagonal(isize, bool, bool),
}

impl Slope {
    // Slope must be a diagonal, or panic
    #[inline]
    fn angle(&self) -> isize {
        match self {
            Slope::Diagonal(x, _, _) => *x,
            _ => panic!("slope.angle() is only valid for Diag"),
        }
    }

    #[inline]
    fn rotation(&self) -> usize {
        match self {
            Slope::Vertical(true) => 0,
            Slope::Diagonal(_, true, false) => 1,
            Slope::Horizontal(false) => 2,
            Slope::Diagonal(_, false, false) => 3,
            Slope::Vertical(false) => 4,
            Slope::Diagonal(_, false, true) => 5,
            Slope::Horizontal(true) => 6,
            Slope::Diagonal(_, true, true) => 7,
        }
    }
}

use std::cmp::Ordering;
impl std::cmp::Ord for Slope {
    fn cmp(&self, other: &Slope) -> Ordering {
        match self.rotation().cmp(&other.rotation()) {
            Ordering::Equal => self.angle().cmp(&other.angle()),
            ord => ord,
        }
    }
}

impl<T> Grid<T> {
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

    pub fn slope(&self, b: Point) -> Slope {
        let dx = self.x as isize - b.x as isize;
        let dy = self.y as isize - b.y as isize;
        if dx == 0 {
            Slope::Vertical(dy > 0)
        } else if dy == 0 {
            Slope::Horizontal(dx > 0)
        } else {
            let s = dy as f64 / dx as f64;
            Slope::Diagonal((s * 1000.0).floor() as isize, dy > 0, dx > 0)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Space {
    Empty,
    Asteroid,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Space::Empty => f.write_str("."),
            Space::Asteroid => f.write_str("#"),
        }
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

fn part1(grid: &Grid<Space>) -> (usize, Point) {
    let mut max = (0, Point::new(0, 0));
    for (base, _) in grid.iter().filter(|(_, &space)| space == Space::Asteroid) {
        let other = grid
            .iter()
            .filter(|(pt, &space)| pt != &base && space == Space::Asteroid)
            .map(|(pt, _)| pt)
            .collect::<Vec<Point>>();
        let mut slopes = HashMap::new();
        for pt in other {
            slopes
                .entry(base.slope(pt))
                .or_insert_with(Vec::new)
                .push(pt);
        }

        if slopes.len() >= max.0 {
            max = (slopes.len(), base)
        }
    }
    max
}

fn part2(grid: &Grid<Space>, base: Point) -> usize {
    let other = grid
        .iter()
        .filter(|(pt, &space)| pt != &base && space == Space::Asteroid)
        .map(|(pt, _)| pt)
        .collect::<Vec<Point>>();
    let mut slopes = HashMap::new();
    for pt in other {
        slopes
            .entry(base.slope(pt))
            .or_insert_with(Vec::new)
            .push(pt);
    }

    let n = grid.rows * grid.cols;
    let mut reachable = Vec::new();
    for (s, on_line) in slopes {
        let nearest = on_line.into_iter().fold((n, Point::default()), |acc, pt| {
            if pt.manhattan(base) < acc.0 {
                (pt.manhattan(base), pt)
            } else {
                acc
            }
        });
        reachable.push((s, nearest.1));
    }
    reachable.sort_by(|a, b| a.0.cmp(&b.0));

    let bet = reachable[199].1;
    bet.x * 100 + bet.y
}

fn parse(input: &str) -> Grid<Space> {
    let mut cols = 0;
    let mut rows = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        cols = 0;
        for ch in line.chars() {
            match ch {
                '.' => grid.push(Space::Empty),
                _ => grid.push(Space::Asteroid),
            }
            cols += 1;
        }
        rows += 1;
    }
    Grid { cols, rows, grid }
}

fn main() {
    let s = std::fs::read_to_string("./day10/input.txt").unwrap();
    let grid = parse(&s);

    let (reach, pt) = part1(&grid);
    println!("Part 1: {} {:?}", reach, pt);
    println!("Part 2: {}", part2(&grid, pt));
}

#[test]
fn example() {
    let s = std::fs::read_to_string("test.txt").unwrap();
    let grid = parse(&s);
    let (reach, pt) = part1(&grid);
    assert_eq!(reach, 210);
    assert_eq!(pt, Point::new(11, 13));
    assert_eq!(part2(&grid, pt), 802);
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
