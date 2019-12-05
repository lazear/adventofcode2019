#[derive(Copy, Clone, Debug)]
enum Mode {
    Position(usize),
    Immediate(isize),
}

#[derive(Copy, Clone, Debug)]
struct Instr {
    op: u16,
    params: [Mode; 3],
}

impl Instr {
    fn aux(mode: u16, param: isize) -> Mode {
        match mode {
            0 => Mode::Position(param as usize),
            1 => Mode::Immediate(param),
            _ => unreachable!(),
        }
    }

    fn new(instr: u16, params: [isize; 3]) -> Instr {
        let a = Instr::aux((instr / 10000) % 10, params[2]);
        let b = Instr::aux((instr / 1000) % 10, params[1]);
        let c = Instr::aux((instr / 100) % 10, params[0]);
        let op = instr % 10;
        Instr {
            op,
            params: [c, b, a],
        }
    }
}

fn main() {
    println!("Hello, world!");
    dbg!(Instr::new(1002, [4, 3, 4]));
}
