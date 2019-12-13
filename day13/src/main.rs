use grid::{Grid, Coord};
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

fn part2(mut vm: Vm) -> isize {
    loop {
        vm.ip = 0;
        vm.data[0] = 2;
        let mut out = Vec::new();

        loop {
            // This ended up being MUCH simpler than I thought, you don't
            // actually even need to supply smart input. I intially tried
            // to track the paddle and ball location.
            match vm.run(std::iter::repeat(0), false) {
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

        let g = Coord::to_grid(map);
        println!("{}", g);


        // pretty_print(map);

        let score = out
            .chunks_exact(3)
            .filter(|x| x[0] == -1 && x[1] == 0)
            .map(|x| x[2])
            .collect::<Vec<_>>();

        if out
            .chunks_exact(3)
            .filter(|x| x[0] != -1 && x[2] == 2)
            .count()
            == 0
        {
            println!("Hello as, world! {:?}", score);
            return score[0];
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./day13/input.txt").unwrap();
    let vm = input.parse::<Vm>().unwrap();
    println!("Part 1: {}", part1(vm.clone()));
    println!("Part 2: {}", part2(vm));
}
