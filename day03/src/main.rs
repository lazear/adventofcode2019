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

#[derive(Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    a: Point,
    b: Point,
}

#[inline]
fn in_interval(interval: (isize, isize), val: isize) -> bool {
    val > interval.0 && val < interval.1
}

impl Line {
    fn contains(&self, pt: Point) -> bool {
        let ix = (self.a.x.min(self.b.x), self.a.x.max(self.b.x));
        let iy = (self.a.y.min(self.b.y), self.a.y.max(self.b.y));
        // (pt.x >= ix.0 && ix.1 >= pt.x) || (ix.0 >= pt.x && pt.x >= ix.1)
        // && (pt.y >= iy.0 && iy.1 >= pt.y) || (iy.0 >= pt.y && pt.y >= iy.1)

        let dxc = pt.x - self.a.x;
        let dyc = pt.y - self.a.y;

        let dxl = self.b.x - self.a.x;
        let dyl = self.b.y - self.b.y;
        let cross = dxc * dyl - dyc * dxl;
        cross == 0
    }

    fn slope(self) -> f32 {
        let dx = (self.b.x - self.a.x) as f32;
        let dy = (self.b.y - self.b.y) as f32;
        dy / dx
    }

    fn intersect(self, other: Line) -> bool {
        false
    }
}

#[derive(Default, Debug)]
struct Grid {
    inner: Vec<Vec<u8>>,
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

impl Grid {
    fn from_directions(dirs: &[Direction]) -> Self {
        // first find dims
        let mut init = Point { x: 0, y: 0 };
        let mut points = vec![init];
        for d in dirs {
            init = init + *d;
            points.push(init);
        }
        let xmin = points.iter().map(|p| p.x).min();
        let ymin = points.iter().map(|p| p.y).min();
        dbg!(ymin);
        dbg!(xmin);
        dbg!(points);
        Grid::default()
    }
}

fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<Direction>, Box<dyn Error>> {
    let s = fs::read_to_string(path)?;
    Ok(s.split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().parse::<Direction>())
        .collect::<Result<_, _>>()?)
}

fn main() {
    let data = parse("./day03/input.txt").unwrap();
    // dbg!(data);
    Grid::from_directions(&data);
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn intersect() {
        let l1 = Line {
            a: Point { x: 0, y: 0 },
            b: Point { x: 5, y: 5 },
        };
        let l2 = Line {
            a: Point { x: 4, y: 2 },
            b: Point { x: 0, y: 9 },
        };
        let l3 = Line {
            a: Point { x: 0, y: 3 },
            b: Point { x: 0, y: 9 },
        };
        assert!(l1.intersect(l2));
        assert!(!l1.intersect(l3));
    }
}
