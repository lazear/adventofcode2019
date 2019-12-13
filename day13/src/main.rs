use grid::{Grid, Point};
use intcode::Vm;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::iter::Iterator;


fn parse(input: &str) -> Option<Vec<isize>> {
    input.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>().ok()
}

fn main() {
    let input = std::fs::read_to_string("./day13/input.txt").unwrap();


    let mut vm = input.parse::<Vm>().unwrap();

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

    let x = out.chunks_exact(3).filter(|s| s[2] == 2).count();

    println!("Hello, world! {}", x);
}
