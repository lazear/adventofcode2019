use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Position(usize),
    Immediate(isize),
    Relative(isize),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    InvalidData,
    InvalidInstr(usize, isize),
    InvalidAddr(usize),
    InvalidMode(usize, isize),
    NoInput,
    Halted,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    Jnz(Mode, Mode),
    Jz(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    Offset(Mode),
    Halt,
}

impl Opcode {
    fn pretty_print(self, vm: &Vm) {
        use Opcode::*;
        match self {
            Add(a, b, c) => println!("[{:?}] = {:?} + {:?}", c, a, b),
            Mul(a, b, c) => println!("[{:?}] = {:?} * {:?}", c, a, b),
            Eq(a, b, c) => println!("[{:?}] = {:?} == {:?}", c, a, b),
            Lt(a, b, c) => println!("[{:?}] = {:?} < {:?}", c, a, b),
            Jnz(a, b) => println!("jnz {:?} {:?}", a, b),
            Jz(a, b) => println!("jz {:?} {:?}", a, b),
            Halt => println!("halt!"),
            Input(cell) => println!("[{:?}] = input", cell),
            Output(cell) => println!("output {:?}", cell),
            Offset(m) => println!("offset {:?}", m),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vm {
    pub data: Vec<isize>,
    pub ip: usize,
    base: usize,
}

impl Vm {
    pub fn new(data: Vec<isize>) -> Self {
        Vm {
            data,
            ip: 0,
            base: 0,
        }
    }

    fn read_param(&mut self, mode_flag: isize) -> Result<Mode, Error> {
        let &a = self.data.get(self.ip).ok_or(Error::InvalidAddr(self.ip))?;
        self.ip += 1;
        match mode_flag {
            0 => Ok(Mode::Position(a as usize)),
            1 => Ok(Mode::Immediate(a)),
            2 => Ok(Mode::Relative(a)),
            _ => Err(Error::InvalidMode(self.ip - 1, mode_flag)),
        }
    }

    fn read_position(&mut self) -> Result<usize, Error> {
        let &a = self.data.get(self.ip).ok_or(Error::InvalidAddr(self.ip))?;
        self.ip += 1;
        Ok(a as usize)
    }

    fn opcode(&mut self) -> Result<Opcode, Error> {
        let &instr = self.data.get(self.ip).ok_or(Error::InvalidAddr(self.ip))?;
        // if instr
        self.ip += 1;
        let a = (instr / 10000) % 10;
        let b = (instr / 1000) % 10;
        let c = (instr / 100) % 10;
        // assert_eq!(a, 0, "third parameter must be in position mode!");
        // if a != 0 {
        //     return Err(Error::InvalidMode(self.ip-1, a));
        // }
        if instr % 100 == 99 {
            return Ok(Opcode::Halt);
        }
        match instr % 10 {
            1 => Ok(Opcode::Add(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_param(a)?,
            )),
            2 => Ok(Opcode::Mul(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_param(a)?,
            )),
            3 => Ok(Opcode::Input(self.read_param(c)?)),
            4 => Ok(Opcode::Output(self.read_param(c)?)),
            5 => Ok(Opcode::Jnz(self.read_param(c)?, self.read_param(b)?)),
            6 => Ok(Opcode::Jz(self.read_param(c)?, self.read_param(b)?)),
            7 => Ok(Opcode::Lt(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_param(a)?,
            )),
            8 => Ok(Opcode::Eq(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_param(a)?,
            )),
            9 => Ok(Opcode::Offset(self.read_param(c)?)),
            _ => Err(Error::InvalidInstr(self.ip - 1, instr)),
        }
    }

    fn fetch(&mut self, mode: Mode) -> Result<isize, Error> {
        match mode {
            Mode::Immediate(i) => Ok(i),
            Mode::Position(idx) => {
                // self.data.get(idx).copied().ok_or(Error::InvalidAddr(idx))
                Ok(self.get_or_extend(idx))
            }
            Mode::Relative(off) => {
                let base = usize::try_from(self.base as isize + off).expect(&format!(
                    "relative addr out of bounds: {}+{}",
                    self.base, off
                ));
                Ok(self.get_or_extend(base))
                // self.data.get(base).copied().ok_or(Error::InvalidAddr(base))
            }
        }
    }

    pub fn decompile(mut self) {
        while self.ip < self.data.len() {
            print!("{}: ", self.ip);
            match self.opcode() {
                Ok(Opcode::Offset(off)) => {
                    self.base =
                        usize::try_from(self.base as isize + self.fetch(off).unwrap()).unwrap()
                }
                Ok(op) => println!("{:?}", op),
                _ => {
                    self.ip += 1;
                }
            }
        }
    }

    fn get_or_extend(&mut self, loc: usize) -> isize {
        if loc >= self.data.len() {
            self.data
                .extend(std::iter::repeat(0).take(2 * (loc - self.data.len() + 1)));
        }
        assert!(loc < self.data.len());
        self.data[loc]
    }

    fn store_or_extend(&mut self, loc: Mode, data: isize) {
        let loc = match loc {
            Mode::Position(x) => x,
            Mode::Relative(off) => usize::try_from(self.base as isize + off).unwrap(),
            _ => unimplemented!(),
        };
        if loc >= self.data.len() {
            self.data
                .extend(std::iter::repeat(0).take(2 * (loc - self.data.len() + 1)));
        }
        assert!(loc < self.data.len());
        self.data[loc] = data;
    }

    pub fn run<I: Iterator<Item = isize>>(
        &mut self,
        mut input: I,
        verbose: bool,
    ) -> Result<isize, Error> {
        while self.ip < self.data.len() {
            let ip = self.ip;
            let op = self.opcode()?;
            assert!(self.ip > ip);

            if verbose {
                print!("{:3}: ", ip);
                op.pretty_print(&self);
            }

            match op {
                Opcode::Halt => {
                    return Err(Error::Halted);
                }
                Opcode::Add(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, a + b);
                }
                Opcode::Mul(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, a * b);
                }
                Opcode::Input(idx) => {
                    self.store_or_extend(idx, input.next().ok_or(Error::NoInput)?);
                }
                Opcode::Output(mode) => {
                    return self.fetch(mode);
                }
                Opcode::Jnz(a, b) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    if a != 0 {
                        self.ip = b as usize;
                    }
                }
                Opcode::Jz(a, b) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    if a == 0 {
                        self.ip = b as usize;
                    }
                }
                Opcode::Lt(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, if a < b { 1 } else { 0 });
                }
                Opcode::Eq(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, if a == b { 1 } else { 0 });
                }
                Opcode::Offset(a) => {
                    let off = self.fetch(a)?;
                    self.base = usize::try_from(self.base as isize + off).unwrap();
                }
            }
        }
        Err(Error::Halted)
    }

    pub fn run_fn<F: FnMut() -> isize>(
        &mut self,
        mut input: F,
        verbose: bool,
    ) -> Result<isize, Error> {
        while self.ip < self.data.len() {
            let ip = self.ip;
            let op = self.opcode()?;
            assert!(self.ip > ip);

            if verbose {
                print!("{:3}: ", ip);
                op.pretty_print(&self);
            }

            match op {
                Opcode::Halt => {
                    return Err(Error::Halted);
                }
                Opcode::Add(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, a + b);
                }
                Opcode::Mul(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, a * b);
                }
                Opcode::Input(idx) => self.store_or_extend(idx, input()),
                Opcode::Output(mode) => {
                    return self.fetch(mode);
                }
                Opcode::Jnz(a, b) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    if a != 0 {
                        self.ip = b as usize;
                    }
                }
                Opcode::Jz(a, b) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    if a == 0 {
                        self.ip = b as usize;
                    }
                }
                Opcode::Lt(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, if a < b { 1 } else { 0 });
                }
                Opcode::Eq(a, b, c) => {
                    let a = self.fetch(a)?;
                    let b = self.fetch(b)?;
                    self.store_or_extend(c, if a == b { 1 } else { 0 });
                }
                Opcode::Offset(a) => {
                    let off = self.fetch(a)?;
                    self.base = usize::try_from(self.base as isize + off).unwrap();
                }
            }
        }
        Err(Error::Halted)
    }
}

impl FromStr for Vm {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split(',')
            .map(|s| s.trim().parse::<isize>().map_err(|_| Error::InvalidData))
            .collect::<Result<_, _>>()?;

        Ok(Vm {
            data,
            ip: 0,
            base: 0,
        })
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
            Ok(Opcode::Mul(
                Mode::Position(4),
                Mode::Immediate(3),
                Mode::Position(4)
            ))
        );
        assert_eq!(
            vm.opcode(),
            Ok(Opcode::Add(
                Mode::Immediate(100),
                Mode::Immediate(-1),
                Mode::Position(4)
            ))
        );
    }

    #[test]
    fn jump_test() {
        let ex = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        vm.clone().decompile();
        assert_eq!(vm.run(std::iter::repeat(7), true), Ok(999));
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(8), true), Ok(1000));
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(9), true), Ok(1001));
    }

    #[test]
    fn relative() {
        let ex = "109,19,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        // vm.clone().decompile();
        assert_eq!(vm.run(std::iter::repeat(0), true), Err(Error::Halted));
        assert_eq!(vm.base, 19);

        let ex = "104,1125899906842624,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(0), true), Ok(1125899906842624));
    }

    #[test]
    fn jnz() {
        // set offset to 9
        // if 4 != 0 then jump to [base -1]
        // output cell 0
        let ex = "109,9,2105,4,-1,4,0,99,5";
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(0), true), Ok(109));
    }
}
