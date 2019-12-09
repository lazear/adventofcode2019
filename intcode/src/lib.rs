use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Position(usize),
    Immediate(isize),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    Add(Mode, Mode, usize),
    Mul(Mode, Mode, usize),
    Input(usize),
    Output(Mode),
    Jnz(Mode, Mode),
    Jz(Mode, Mode),
    Lt(Mode, Mode, usize),
    Eq(Mode, Mode, usize),
    Halt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vm {
    data: Vec<isize>,
    ip: usize,
}

impl Vm {
    pub fn new(data: Vec<isize>) -> Self {
        Vm { data, ip: 0 }
    }

    fn read_param(&mut self, mode_flag: isize) -> Option<Mode> {
        let &a = self.data.get(self.ip)?;
        self.ip += 1;
        match mode_flag {
            0 => Some(Mode::Position(a as usize)),
            1 => Some(Mode::Immediate(a)),
            _ => None, //panic!("invalid instruction decoding!"),
        }
    }

    fn read_position(&mut self) -> Option<usize> {
        let &a = self.data.get(self.ip)?;
        self.ip += 1;
        Some(a as usize)
    }

    fn opcode(&mut self) -> Option<Opcode> {
        let &instr = self.data.get(self.ip)?;
        // if instr
        self.ip += 1;
        let a = (instr / 10000) % 10;
        let b = (instr / 1000) % 10;
        let c = (instr / 100) % 10;
        // assert_eq!(a, 0, "third parameter must be in position mode!");
        if a != 0 {
            return None;
        }
        let op = instr % 10;
        match op {
            1 => Some(Opcode::Add(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            2 => Some(Opcode::Mul(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            3 => Some(Opcode::Input(self.read_position()?)),
            4 => Some(Opcode::Output(self.read_param(c)?)),
            5 => Some(Opcode::Jnz(self.read_param(c)?, self.read_param(b)?)),
            6 => Some(Opcode::Jz(self.read_param(c)?, self.read_param(b)?)),
            7 => Some(Opcode::Lt(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            8 => Some(Opcode::Eq(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            9 => Some(Opcode::Halt),
            _ => None,
        }
    }

    fn fetch(&self, mode: Mode) -> isize {
        match mode {
            Mode::Immediate(i) => i,
            Mode::Position(idx) => self.data[idx],
        }
    }

    pub fn decompile(mut self) {
        while self.ip < self.data.len() {
            print!("{}: ", self.ip);
            match self.opcode() {
                Some(Opcode::Jz(a, b)) => {
                    println!("{:?}", Opcode::Jz(a, b));
                    if self.fetch(a) == 0 {
                        self.ip = self.fetch(b) as usize;
                    }
                }
                Some(Opcode::Jnz(a, b)) => {
                    println!("{:?}", Opcode::Jnz(a, b));
                    if self.fetch(a) != 0 {
                        self.ip = self.fetch(b) as usize;
                    }
                }
                Some(op) => println!("{:?}", op),
                _ => {
                    self.ip += 1;
                }
            }
        }
    }

    pub fn run<I: Iterator<Item = isize>>(&mut self, mut input: I) -> Option<isize> {
        self.ip = 0;
        loop {
            match self.opcode() {
                Some(Opcode::Halt) => break,
                Some(Opcode::Add(a, b, c)) => {
                    self.data[c] = self.fetch(a) + self.fetch(b);
                }
                Some(Opcode::Mul(a, b, c)) => {
                    self.data[c] = self.fetch(a) * self.fetch(b);
                }
                Some(Opcode::Input(idx)) => {
                    self.data[idx] = input.next()?;
                }
                Some(Opcode::Output(mode)) => {
                    if self.data.get(self.ip)? == &99 {
                        return Some(self.fetch(mode));
                    } else {
                        if self.fetch(mode) != 0 {
                            return Some(self.fetch(mode));
                        }
                        println!("out: {}", self.fetch(mode));
                    }
                }
                Some(Opcode::Jnz(a, b)) => {
                    if self.fetch(a) != 0 {
                        self.ip = self.fetch(b) as usize;
                    }
                }
                Some(Opcode::Jz(a, b)) => {
                    if self.fetch(a) == 0 {
                        self.ip = self.fetch(b) as usize;
                    }
                }
                Some(Opcode::Lt(a, b, c)) => {
                    self.data[c] = if self.fetch(a) < self.fetch(b) { 1 } else { 0 }
                }
                Some(Opcode::Eq(a, b, c)) => {
                    self.data[c] = if self.fetch(a) == self.fetch(b) { 1 } else { 0 }
                }
                None => panic!("Invalid instruction!"),
            }
        }
        None
    }
}

impl FromStr for Vm {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split(',')
            .map(|s| s.trim().parse::<isize>())
            .collect::<Result<_, _>>()?;

        Ok(Vm { data, ip: 0 })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn decoding() {
        let instr = vec![1002, 4, 3, 4, 1101, 100, -1, 4, 0];
        let mut vm = Vm::new(instr);
        assert_eq!(
            vm.opcode(),
            Some(Opcode::Mul(Mode::Position(4), Mode::Immediate(3), 4))
        );
        assert_eq!(
            vm.opcode(),
            Some(Opcode::Add(Mode::Immediate(100), Mode::Immediate(-1), 4))
        );
    }

    #[test]
    fn jump_test() {
        let ex = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(7)), Some(999));
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(8)), Some(1000));
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(9)), Some(1001));
    }
}
