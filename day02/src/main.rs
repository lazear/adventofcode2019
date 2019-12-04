use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum Opcode {
    // Opcode 1 adds together numbers read from 2 positions and stores the
    // results in a third position
    Add(usize, usize, usize),
    // Opcode 2 works exactly like opcode 1, except it multiples the two
    // inputs
    Mul(usize, usize, usize),
    // Opcode 99 halts
    Halt,
}

pub fn parse(s: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    Ok(s.split(',')
        .map(|s| s.trim().parse::<usize>())
        .collect::<Result<_, _>>()?)
}
struct Vm {
    data: Vec<usize>,
    ip: usize,
}

impl Vm {
    pub fn new(data: Vec<usize>) -> Self {
        Vm { data, ip: 0 }
    }

    fn read3(&self) -> Option<(usize, usize, usize)> {
        let &a = self.data.get(self.ip + 1)?;
        let &b = self.data.get(self.ip + 2)?;
        let &c = self.data.get(self.ip + 3)?;
        Some((a, b, c))
    }

    fn opcode(&self) -> Option<Opcode> {
        match self.data[self.ip] {
            1 => self.read3().map(|(a, b, c)| Opcode::Add(a, b, c)),
            2 => self.read3().map(|(a, b, c)| Opcode::Mul(a, b, c)),
            99 => Some(Opcode::Halt),
            _ => None,
        }
    }

    fn run(mut self) -> Vec<usize> {
        loop {
            match self.opcode() {
                Some(Opcode::Halt) => break,
                Some(Opcode::Add(a, b, c)) => {
                    self.data[c] = self.data[a] + self.data[b];
                    self.ip += 4;
                }
                Some(Opcode::Mul(a, b, c)) => {
                    self.data[c] = self.data[a] * self.data[b];
                    self.ip += 4;
                }
                None => panic!("Invalid instruction!"),
            }
        }
        self.data
    }

    // Well... I was gonna make a dominator graph, but brute forcing was faster...
    // fn reverse_engineer(mut self) -> Vec<usize> {
    //     let mut ops = Vec::new();
    //     loop {
    //         match self.opcode() {
    //             Some(Opcode::Halt) => break,
    //             Some(op) => {
    //                 ops.push(op);
    //                 self.ip += 4
    //             }
    //             None => panic!("Invalid instruction!"),
    //         }
    //     }

    //     let mut outputs = HashSet::new();
    //     outputs.insert(1);
    //     outputs.insert(2);
    //     let mut edges = Vec::new();

    //     let mut deps: HashMap<usize, Vec<Opcode>> = HashMap::new();

    //     // for op in ops.iter().rev() {
    //     //     match op {
    //     //         Opcode::Add(a, b, c) | Opcode::Mul(a, b, c) => {
    //     //             if outputs.contains(c) {
    //     //                 outputs.insert(*a);
    //     //                 outputs.insert(*b);
    //     //                 edges.push(op);
    //     //             }
    //     //         }
    //     //         _ => {}
    //     //     }
    //     // }
    //     //
    //     for op in ops.iter() {
    //         match op {
    //             Opcode::Add(a, b, c) | Opcode::Mul(a, b, c) => {
    //                 if outputs.contains(a) || outputs.contains(b) {
    //                     println!("{:?}", op);
    //                     outputs.insert(*c);
    //                     // outputs.insert(*b);
    //                     edges.push(op);
    //                     deps.entry(*c).or_insert_with(Vec::new).push(*op);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }

    //     let mut out = 0;
    //     loop {
    //         let ins = deps.get(out).unwrap();

    //     }

    //     // self.ip = ops.len() * 4 - 4;
    //     // loop {
    //     //     match ops.pop() {
    //     //         Opcode::Add(a, b, c) => {
    //     //             self.data[c]
    //     //         }
    //     //     }
    //     // }

    //     self.data
    // }
}

fn part1<P: AsRef<Path>>(path: P) -> Result<usize, Box<dyn Error>> {
    let s = fs::read_to_string(path)?;
    let mut data = parse(&s)?;
    data[1] = 12;
    data[2] = 2;
    let res = Vm::new(data).run();
    Ok(res[0])
}

fn part2<P: AsRef<Path>>(path: P) -> Result<usize, Box<dyn Error>> {
    let s = fs::read_to_string(path)?;
    let mut data = parse(&s)?;

    for i in 0..99 {
        for j in 0..99 {
            let mut d = data.clone();
            d[1] = i;
            d[2] = j;
            let res = Vm::new(d).run();
            if res[0] == 19690720 {
                return Ok(100 * i + j);
            }
        }
    }

    Ok(0)
}

fn main() {
    println!("Part 1: {}", part1("./day02/input.txt").unwrap());
    println!("Part 1: {}", part2("./day02/input.txt").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vm() {
        assert_eq!(
            Vm::new(parse("1,9,10,3,2,3,11,0,99,30,40,50").unwrap()).run(),
            parse("3500,9,10,70,2,3,11,0,99,30,40,50").unwrap()
        );
        assert_eq!(
            Vm::new(parse("1,0,0,0,99").unwrap()).run(),
            parse("2,0,0,0,99").unwrap()
        );
        assert_eq!(
            Vm::new(parse("2,3,0,3,99").unwrap()).run(),
            parse("2,3,0,6,99").unwrap()
        );
        assert_eq!(
            Vm::new(parse("1,1,1,4,99,5,6,0,99").unwrap()).run(),
            parse("30,1,1,4,2,5,6,0,99").unwrap()
        );
    }
}
