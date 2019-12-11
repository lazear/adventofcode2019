use grid::{Grid, Point};
use std::collections::HashMap;

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

fn slope(a: Point, b: Point) -> Slope {
    let dx = a.x as isize - b.x as isize;
    let dy = a.y as isize - b.y as isize;
    if dx == 0 {
        Slope::Vertical(dy > 0)
    } else if dy == 0 {
        Slope::Horizontal(dx > 0)
    } else {
        let s = dy as f64 / dx as f64;
        Slope::Diagonal((s * 1000.0).floor() as isize, dy > 0, dx > 0)
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
                .entry(slope(base, pt))
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
            .entry(slope(base, pt))
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
    Grid::new(cols, rows, grid)
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
