use intcode::Vm;
use std::iter::Iterator;

struct Iter {
    phase: isize,
    input: isize,
    state: isize,
}

impl Iter {
    pub fn new(phase: isize, input: isize) -> Self {
        Iter {
            phase,
            input,
            state: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let r = match self.state {
            0 => Some(self.phase),
            1 => Some(self.input),
            _ => None,
        };
        self.state += 1;
        r
    }
}

fn phrase(vm: &Vm, phrase: &[isize]) -> isize {
    phrase.into_iter().fold(0, |acc, ph| {
        vm.clone()
            .run(Iter {
                phase: *ph,
                input: acc,
                state: 0,
            })
            .unwrap()
    })
}

fn heaps(slice: &mut [isize], n: usize, out: &mut Vec<Vec<isize>>) {
    if n == 1 {
        out.push(slice.iter().copied().collect());
    } else {
        for i in 0..n {
            heaps(slice, n - 1, out);
            if n % 2 == 1 {
                slice.swap(0, n - 1);
            } else {
                slice.swap(i, n - 1);
            }
        }
    }
}

fn part1(input: &str) -> Option<isize> {
    let vm = input.parse::<Vm>().ok()?;
    let mut max = 0;
    let mut v = Vec::new();
    heaps(&mut [0, 1, 2, 3, 4], 5, &mut v);
    for p in v {
        let x = phrase(&vm, &p);
        if x > max {
            max = x;
        }
    }

    Some(max)
}

fn run_loop(mut vms: Vec<Vm>, phase: &[isize]) -> isize {
    let mut acc = 0;
    let mut max = 0;

    while let Ok(x) = vms
        .iter_mut()
        .zip(phase)
        .try_fold(acc, |acc, (v, &phase)| v.run(Iter::new(phase, acc)))
    {
        max = max.max(x);
        acc = x;
    }

    println!("loop complete");
    acc
}

fn part2(input: &str) -> Option<isize> {
    let vm = input.parse::<Vm>().ok()?;
    let vms = std::iter::repeat(vm.clone()).take(5).collect::<Vec<_>>();

    let mut max = 0;

    let mut v = Vec::new();
    heaps(&mut [5, 6, 7, 8, 9], 5, &mut v);

    for p in &v {
        let x = run_loop(vms.clone(), p);
        println!("{}", x);
    }

    Some(max)
}

fn main() {
    let input = std::fs::read_to_string("./day07/input.txt").unwrap();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

#[test]
fn examples_part1() {
    assert_eq!(
        phrase(
            &"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
                .parse::<Vm>()
                .unwrap(),
            &[4, 3, 2, 1, 0]
        ),
        43210
    );
    assert_eq!(
        phrase(
            &"3,23,3,24,1002,24,10,24,1002,23,-1,23,
    101,5,23,23,1,24,23,23,4,23,99,0,0"
                .parse::<Vm>()
                .unwrap(),
            &[0, 1, 2, 3, 4]
        ),
        54321
    );
    assert_eq!(
        phrase(
            &"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
    1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
                .parse::<Vm>()
                .unwrap(),
            &[1, 0, 4, 3, 2]
        ),
        65210
    );
}

#[test]
fn examples_part2() {
    fn harness(input: &str, phrase: &[isize]) -> Option<isize> {
        let mut vm = input.parse::<Vm>().ok()?;
        let mut max = 0;
        let mut x = 0;

        'outer: loop {
            for ph in phrase {
                x = match vm.run(Iter {
                    phase: *ph,
                    input: x,
                    state: 0,
                }) {
                    Ok(x) => dbg!(x),
                    Err(e) => {
                        println!("fail {:?}", &phrase);
                        break 'outer;
                    }
                };
                if x > max {
                    max = x;
                }
            }
        }

        Some(max)
    }

    // assert_eq!(
    //     harness(
    //         &"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
    //         27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    //         &[9, 8, 7, 6, 5]
    //     ),
    //     Some(139629729)
    // );
}
