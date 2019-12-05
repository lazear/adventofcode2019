use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug, PartialEq)]
enum Perp {
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Line {
    a: Point,
    b: Point,
}

#[inline]
fn in_interval(a: isize, b: isize, val: isize) -> bool {
    let max = a.max(b);
    let min = a.min(b);
    (val >= min && val <= max)
}

impl Line {
    fn perp(self) -> Perp {
        if self.a.x == self.b.x {
            Perp::Vertical
        } else {
            Perp::Horizontal
        }
    }

    fn contains(self, pt: Point) -> bool {
        match self.perp() {
            Perp::Vertical => in_interval(self.a.y, self.b.y, pt.y),
            Perp::Horizontal => in_interval(self.a.x, self.b.x, pt.x),
        }
    }

    fn intersect(self, other: Line) -> Option<Point> {
        if self.perp() == other.perp() {
            return None;
        }

        let xmin = self.a.x.min(self.b.x);
        let xmax = self.a.x.max(self.b.x);
        let ymin = self.a.y.min(self.b.y);
        let ymax = self.a.y.max(self.b.y);

        match self.perp() {
            Perp::Vertical => {
                assert_eq!(other.a.y, other.b.y);
                assert_eq!(self.a.x, self.b.x);
                if (other.a.y >= ymin && other.a.y <= ymax)
                    && (self.a.x >= other.a.x.min(other.b.x)
                        && self.a.x <= other.a.x.max(other.b.x))
                {
                    // other must be horizontal, so it's Xs vary
                    for y in ymin..=ymax {
                        if y == other.a.y {
                            println!("{:?} {:?} {} {} {}", self, other, ymin, ymax, y);
                            return Some(Point { x: self.a.x, y });
                        }
                    }
                }
            }
            Perp::Horizontal => {
                assert_eq!(other.a.x, other.b.x);
                assert_eq!(self.a.y, self.b.y);
                if (other.a.x >= xmin && other.a.x <= xmax)
                    && (self.a.y >= other.a.y.min(other.b.y)
                        && self.a.y <= other.a.y.max(other.b.y))
                {
                    // other must be vertical, so it's Ys vary
                    for x in xmin..=xmax {
                        if x == other.a.x {
                            return Some(Point { x, y: self.a.y });
                        }
                    }
                }
            }
        }
        None
    }
}

impl Point {
    fn dist(self, other: Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl FromStr for Direction {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s[1..].parse::<isize>()?;
        match &s[0..1] {
            "R" => Ok(Direction::Right(i)),
            "L" => Ok(Direction::Left(i)),
            "U" => Ok(Direction::Up(i)),
            "D" => Ok(Direction::Down(i)),
            _ => panic!("Invalid direction! {}", s),
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        let (x, y) = match rhs {
            Up(d) => (self.x, self.y - d),
            Down(d) => (self.x, self.y + d),
            Right(d) => (self.x + d, self.y),
            Left(d) => (self.x - d, self.y),
        };
        Point { x, y }
    }
}

fn to_lines(dirs: &[Direction]) -> Vec<Line> {
    // first find dims
    let mut a = Point { x: 0, y: 0 };
    let mut lines = vec![];
    for d in dirs {
        let b = a + *d;
        lines.push(Line { a, b });
        a = b;
    }
    lines
}

fn intersects(a: &[Line], b: &[Line]) -> HashSet<Point> {
    let mut pts = HashSet::new();
    for i in a {
        for j in b {
            if let Some(pt) = i.intersect(*j) {
                // println!("{:?} ... {:?} {:?}", pt, i, j);
                pts.insert(pt);
            }
        }
    }
    pts.remove(&Point { x: 0, y: 0 });
    pts
}

fn parse<P: AsRef<Path>>(path: P) -> Result<(Vec<Direction>, Vec<Direction>), Box<dyn Error>> {
    let s = fs::read_to_string(path)?;
    let mut lines = s
        .lines()
        .map(|s| {
            s.split(',')
                .map(|s| s.trim().parse::<Direction>())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let a = lines.remove(0);
    let b = lines.remove(0);
    Ok((a, b))
}

fn part1<P: AsRef<Path>>(path: P) -> Result<usize, Box<dyn Error>> {
    let (wa, wb) = parse(path)?;
    let la = to_lines(&wa);
    let lb = to_lines(&wb);
    let pts = intersects(&la, &lb);
    let origin = Point { x: 0, y: 0 };
    let closest = pts.into_iter().map(|pt| pt.dist(origin)).min().unwrap();
    Ok(closest)
}

fn main() {
    println!("Part 1: {}", part1("./day03/input.txt").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn intersect() {
        let l1 = Line {
            a: Point { x: 0, y: 0 },
            b: Point { x: 5, y: 0 },
        };
        let l2 = Line {
            a: Point { x: 5, y: 5 },
            b: Point { x: 5, y: -1 },
        };
        assert_eq!(l1.intersect(l2), Some(Point { x: 5, y: 0 }));
        assert_eq!(l2.intersect(l1), Some(Point { x: 5, y: 0 }));
    }

    #[test]
    fn test() {
        let (wa, wb) = parse("../day03/test.txt").unwrap();
        let la = to_lines(&wa);
        let lb = to_lines(&wb);
        let pts = intersects(&la, &lb);
        assert_eq!(
            pts,
            vec![Point { x: 3, y: -3 }, Point { x: 6, y: -5 }]
                .into_iter()
                .collect()
        );
    }
}
