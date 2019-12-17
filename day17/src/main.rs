use grid::{Coord, Direction, Grid, Point, Rotation};
use intcode::Vm;
use std::collections::HashMap;
use std::fmt::Display;

fn alignment(grid: &Grid<Position>, pt: Point) -> Option<usize> {
    use Direction::*;
    if grid[pt] == Position::Scaffold {
        let neighbors = [
            pt.move_one(Down),
            pt.move_one(Left),
            pt.move_one(Right),
            pt.move_one(Up),
        ];

        let c = neighbors
            .iter()
            .filter(|&&p| p != pt && p.x < grid.cols && p.y < grid.rows)
            .filter(|&&p| grid[p] == Position::Scaffold)
            .count();

        if c == 4 {
            Some(pt.x * pt.y)
        } else {
            None
        }
    } else {
        None
    }
}

fn position_grid(mut vm: Vm) -> Grid<Position> {
    let mut data = HashMap::new();
    let mut last = Coord::new(0, 0);
    while let Ok(out) = vm.run(std::iter::empty(), false) {
        let c = out as u8 as char;
        if c == '\n' {
            last.y += 1;
            last.x = 0;
        } else {
            let pos = match c {
                '#' => Position::Scaffold,
                '.' => Position::Empty,
                '^' => Position::Robot(Direction::Up),
                _ => panic!("unknown {}", c),
            };
            data.insert(last, pos);
            last.x += 1;
        }
    }

    Coord::to_grid(data)
}

fn find_end(grid: &Grid<Position>) -> Option<Point> {
    use Direction::*;
    for (pt, pos) in grid.iter() {
        match pos {
            Position::Scaffold => {
                let neighbors = [
                    pt.move_one(Down),
                    pt.move_one(Left),
                    pt.move_one(Right),
                    pt.move_one(Up),
                ];

                if neighbors
                    .iter()
                    .filter(|&&p| p != pt && p.x < grid.cols && p.y < grid.rows)
                    .filter(|&&p| grid[p] != Position::Empty)
                    .count()
                    == 1
                {
                    return Some(pt);
                }
            }
            _ => {}
        }
    }
    None
}

#[derive(Clone)]
struct Tracer<'g> {
    dir: Direction,
    pt: Point,
    grid: &'g Grid<Position>,
    moves: Vec<Move>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Move {
    rotation: Rotation,
    steps: usize,
}

impl<'g> Tracer<'g> {
    fn max_path(&self, dir: Direction) -> usize {
        let mut pt = self.pt;
        let mut steps: usize = 0;
        while self.grid[pt] != Position::Empty {
            if (pt.x == 0 && dir == Direction::Left) || (pt.y == 0 && dir == Direction::Up) {
                return steps;
            }
            pt = pt.move_one(dir);
            steps += 1;
            if !self.grid.in_bounds(pt) {
                break;
            }
        }
        steps.saturating_sub(1)
    }

    fn find_best_move(&self) -> Move {
        let r = self.max_path(self.dir.rotate(Rotation::Right));
        let l = self.max_path(self.dir.rotate(Rotation::Left));
        if r > l {
            Move {
                rotation: Rotation::Right,
                steps: r,
            }
        } else {
            Move {
                rotation: Rotation::Left,
                steps: l,
            }
        }
    }

    fn perform_move(&mut self, mv: Move) {
        self.dir = self.dir.rotate(mv.rotation);
        for _ in 0..mv.steps {
            self.pt = self.pt.move_one(self.dir);
        }
        self.moves.push(mv);
    }

    fn run(grid: &Grid<Position>) -> Option<Vec<Move>> {
        let start = grid
            .iter()
            .filter(|(_, pos)| {
                if let Position::Robot(_) = pos {
                    true
                } else {
                    false
                }
            })
            .map(|(pt, _)| pt)
            .next()
            .unwrap();
        let end = find_end(&grid).unwrap();

        let mut t = Tracer {
            dir: Direction::Up,
            pt: start,
            grid: &grid,
            moves: Vec::new(),
        };

        while t.pt != end {
            let mv = t.find_best_move();
            t.perform_move(mv);
        }

        Some(t.moves)
    }
}

fn part1(vm: Vm) -> usize {
    let grid = position_grid(vm);
    grid.iter()
        .filter(|(_, &ch)| ch == Position::Scaffold)
        .filter_map(|(pt, _)| alignment(&grid, pt))
        .sum::<usize>()
}

#[derive(Debug)]
struct Program {
    main: String,
    routines: HashMap<char, String>,
}

fn compile(moves: Vec<Move>) -> Option<Program> {
    let mut s = moves
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let mut savings: HashMap<usize, Vec<String>> = HashMap::new();
    for i in 2..5 {
        let strings = moves
            .windows(i)
            .filter_map(|slice| {
                let s = slice
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                if s.len() < 19 {
                    Some(s)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        let mut map = HashMap::new();
        for s in strings {
            *map.entry(s).or_insert(0) += 1;
        }

        for (k, occur) in map {
            savings
                .entry(occur * i * k.len())
                .or_insert_with(Vec::new)
                .push(k);
        }
    }

    let mut k = savings.keys().copied().collect::<Vec<usize>>();
    k.sort();

    let mut routines = HashMap::new();

    for i in 0..3 {
        let ch = (i as u8 + 'A' as u8) as char;

        // We may have already removed the full substring, so loop until we
        // find a full match
        loop {
            let key = k.pop()?;
            let sub = savings.get(&key)?.last()?;
            let n = s.replace(sub, &format!("{}", ch));
            if n != s {
                s = n;
                routines.insert(ch, format!("{}\n", sub));
                break;
            }
        }
    }

    Some(Program {
        main: format!("{}\n", s),
        routines,
    })
}

fn part2(mut vm: Vm) -> Option<isize> {
    let grid = position_grid(vm.clone());
    let moves = Tracer::run(&grid)?;
    let prog = compile(moves)?;

    println!("{:?}", prog);

    vm.data[0] = 2;

    let a = prog.routines.get(&'A')?.bytes();
    let b = prog.routines.get(&'B')?.bytes();
    let c = prog.routines.get(&'C')?.bytes();

    let mut iter = prog
        .main
        .bytes()
        .chain(a)
        .chain(b)
        .chain(c)
        .chain(std::iter::once('n' as u8))
        .chain(std::iter::once('\n' as u8));

    let mut last = 0;
    loop {
        match vm.run_fn(|| iter.next().unwrap_or(0) as u8 as isize, false) {
            Ok(x) => last = x,
            Err(e) => {
                dbg!(e);
                vm.ip -= 2;
                break;
            }
        }
    }
    Some(last)
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Position {
    Empty,
    Scaffold,
    Robot(Direction),
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::Empty => '.',
                Position::Scaffold => '#',
                Position::Robot(dir) => match dir {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                },
            }
        )
    }
}

impl Default for Position {
    fn default() -> Position {
        Position::Empty
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{},{}",
            match self.rotation {
                Rotation::Left => 'L',
                Rotation::Right => 'R',
            },
            self.steps
        )
    }
}

fn main() {
    let input = std::fs::read_to_string("./day17/input.txt").unwrap();
    let vm = input.parse::<Vm>().unwrap();
    println!("Part 1: {}", part1(vm.clone()));
    println!("Part 2: {:?}", part2(vm));
}
