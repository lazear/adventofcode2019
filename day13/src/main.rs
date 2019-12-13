use grid::{Grid, Point};
use intcode::Vm;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::iter::Iterator;

fn parse(input: &str) -> Option<Vec<isize>> {
    input
        .split(',')
        .map(|s| s.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

struct Iter {
    ball: Coord,
    paddle: Coord,
}

impl Iterator for Iter {
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        Some(if self.ball.x > self.paddle.x {
            1
        } else if self.ball.x == self.paddle.x {
            0
        } else {
            1
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

fn pretty_print(colors: HashMap<Coord, isize>) {
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
        grid[pt] = (color as u8 + '0' as u8) as char;
    }
    println!("{}", grid);
}

fn main() {
    let input = std::fs::read_to_string("./day13/input.txt").unwrap();

    let mut vm = input.parse::<Vm>().unwrap();
    vm.data[0] = 2;
    // let mut out = Vec::new();

    // loop {

    //     match vm.run(Iter { ball, paddle}, false) {
    //         Ok(r) => out.push(r),
    //         Err(e) => {
    //             // dbg!(e);
    //             break;
    //         }
    //     }
    // }

    // // let x =

    // let dimx = objs.iter().map(|x| x.0.x).max().unwrap() as usize;
    // let dimy = objs.iter().map(|x| x.0.y).max().unwrap();

    // dbg!(dimx, dimy);

    // let mut g = Grid::new(dimx, dimy, std::iter::repeat(7).take(dimx*dimy).collect());
    // let mut vm = input.parse::<Vm>().unwrap();
    //
    loop {
        vm.ip = 0;
        vm.data[0] = 2;
        let mut out = Vec::new();
        let mut ball = Coord::new(0, 0);
        let mut paddle = Coord::new(0, 0);

        loop {
            match vm.run(Iter { ball, paddle }, false) {
                Ok(r) => out.push(r),
                Err(e) => {
                    // dbg!(e);
                    break;
                }
            }
        }

        let objs = out
            .chunks_exact(3)
            .map(|x| (Coord::new(x[0], x[1]), x[2]))
            .collect::<Vec<_>>();

        let mut map = HashMap::new();
        for (pt, o) in objs {
            map.insert(pt, o);
        }

        pretty_print(map);
        // dbg!(&out);
        ball = out
            .chunks_exact(3)
            .filter(|x| x[2] == 4)
            .map(|x| Coord::new(x[0], x[1]))
            .next()
            .unwrap_or_else(|| Coord::new(0, 0));
        paddle = out
            .chunks_exact(3)
            .filter(|x| x[2] == 3)
            .map(|x| Coord::new(x[0], x[1]))
            .next()
            .unwrap_or_else(|| Coord::new(0, 0));
        // dbg!(ball, paddle);

        let mut score = out
        .chunks_exact(3)
        .filter(|x| x[0] == -1 && x[1] == 0)
        .map(|x| x[2])
        .collect::<Vec<_>>();

        if out
        .chunks_exact(3)
        .filter(|x| x[0] != -1 && x[2] == 2)
        .count() == 0
        {
            
            println!("Hello as, world! {:?}", score);
            break;

            // break;
        }
    }

    // println!("Hello, world! {}", g);
}
