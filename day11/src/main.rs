use grid::{Grid, Point};
use intcode::Vm;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::iter::Iterator;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    pub fn value(self) -> isize {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Rotation {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn step(self, dir: Direction) -> Coord {
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

fn step(vm: &mut Vm, input: Color) -> Option<(Color, Rotation)> {
    let c = match vm.run(std::iter::once(input.value()), false).ok()? {
        0 => Color::Black,
        1 => Color::White,
        c => panic!("invalid color {}", c),
    };
    let d = match vm.run(std::iter::empty(), false).ok()? {
        0 => Rotation::Left,
        1 => Rotation::Right,
        c => panic!("invalid rotation {}", c),
    };
    Some((c, d))
}

fn pretty_print(last: Coord, colors: HashMap<Coord, Color>, dir: Direction) {
    let min_x = colors.keys().map(|c| c.x).min().unwrap();
    let min_y = colors.keys().map(|c| c.y).min().unwrap();
    let max_x = colors.keys().map(|c| c.x).max().unwrap();
    let max_y = colors.keys().map(|c| c.y).max().unwrap();

    let cols = usize::try_from(max_x - min_x).unwrap() + 1;
    let rows = usize::try_from(max_y - min_y).unwrap() + 1;
    let g = std::iter::repeat('.')
        .take(cols * rows)
        .collect::<Vec<char>>();
    let mut grid = Grid::new(cols, rows, g);

    for (coord, color) in colors {
        let pt = Point::new(
            (coord.x + min_x.abs()) as usize,
            (coord.y + min_y.abs()) as usize,
        );
        grid[pt] = match color {
            Color::Black => '.',
            Color::White => '#',
        };
    }
    let pt = Point::new(
        (last.x + min_x.abs()) as usize,
        (last.y + min_y.abs()) as usize,
    );
    grid[pt] = match dir {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    };
    println!("{}", grid);
}

fn problem(mut vm: Vm, initial_color: Color, print_grid: bool) -> usize {
    let mut colors = HashMap::new();
    let mut dir = Direction::Up;
    let mut position = Coord { x: 0, y: 0 };
    let mut on = initial_color;

    while let Some((c, r)) = step(&mut vm, on) {
        colors.insert(position, c);
        dir = dir.rotate(r);
        position = position.step(dir);
        on = *colors.get(&position).unwrap_or(&Color::Black);
    }
    let n = colors.len();
    if print_grid {
        pretty_print(position, colors, dir);
    }
    n
}

fn main() {
    let input = std::fs::read_to_string("./day11/input.txt").unwrap();
    let vm = input.parse::<Vm>().unwrap();

    println!("Part 1: {}", problem(vm.clone(), Color::Black, false));
    println!("Part 1: {}", problem(vm.clone(), Color::White, true));
}
