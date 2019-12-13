use grid::{Coord, Grid};
use intcode::Vm;
use std::collections::HashMap;
use std::iter::Iterator;

fn part1(mut vm: Vm) -> usize {
    vm.data[0] = 2;
    let mut out = Vec::new();
    loop {
        match vm.run(std::iter::repeat(0), false) {
            Ok(r) => out.push(r),
            Err(e) => {
                dbg!(e);
                break;
            }
        }
    }
    out.chunks_exact(3).filter(|s| s[2] == 2).count()
}

struct Iter {
    ball: Coord,
    paddle: Coord,
}

impl Iterator for Iter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let x = match self.ball.x.cmp(&self.paddle.x) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
        };
        Some(x)
    }
}

fn part2(mut vm: Vm, animate: bool) -> isize {
    vm.data[0] = 2;
    let mut ball = Coord::default();
    let mut paddle = Coord::default();
    let mut score = 0;

    let mut out = Vec::new();

    let mut field = HashMap::new();
    loop {
        for slice in out.chunks_exact(3) {
            match slice[2] {
                3 => paddle = Coord::new(slice[0], slice[1]),
                4 => ball = Coord::new(slice[0], slice[1]),
                x if (slice[0] == -1 && slice[1] == 0) => score = x,
                x => {
                    field.insert(Coord::new(slice[0], slice[1]), slice[2]);
                }
            }
        }

        if out.len() == 3 {
            out.clear();
        }

        match vm.run(Iter { ball, paddle }, false) {
            Ok(r) => out.push(r),
            Err(intcode::Error::Halted) => {
                break;
            }
            Err(e) => {
                dbg!(e);
                return -1;
            }
        }

        if animate && paddle != Coord::default() {
            field.insert(paddle, 3);
            field.insert(ball, 4);
            let g = Coord::to_grid(field.clone());
            field.remove(&paddle);
            field.remove(&ball);
            println!("\n\n{}", g);
        }
    }
    return score;
}

fn main() {
    let input = std::fs::read_to_string("./day13/input.txt").unwrap();
    let vm = input.parse::<Vm>().unwrap();
    println!("Part 1: {}", part1(vm.clone()));
    println!("Part 2: {}", part2(vm, true));
}
