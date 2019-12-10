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
    InvalidInstr,
    InvalidAddr,
    InvalidMode,
    NoInput,
    Halted,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    Add(Mode, Mode, usize),
    Mul(Mode, Mode, usize),
    Input(Mode),
    Output(Mode),
    Jnz(Mode, Mode),
    Jz(Mode, Mode),
    Lt(Mode, Mode, usize),
    Eq(Mode, Mode, usize),
    Offset(isize),
    Halt,
}

impl Opcode {
    fn pretty_print(self, vm: &Vm) {
        use Opcode::*;
        match self {
            Add(a, b, c) => println!("[{}] = {:?} + {:?}", c, a, b),
            Mul(a, b, c) => println!("[{}] = {:?} * {:?}", c, a, b),
            Eq(a, b, c) => println!("[{}] = {:?} == {:?}", c, a, b),
            Lt(a, b, c) => println!("[{}] = {:?} < {:?}", c, a, b),
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
    ip: usize,
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
        let &a = self.data.get(self.ip).ok_or(Error::InvalidAddr)?;
        self.ip += 1;
        match mode_flag {
            0 => Ok(Mode::Position(a as usize)),
            1 => Ok(Mode::Immediate(a)),
            2 => Ok(Mode::Relative(a)),
            _ => Err(Error::InvalidMode),
        }
    }

    fn read_position(&mut self) -> Result<usize, Error> {
        let &a = self.data.get(self.ip).ok_or(Error::InvalidAddr)?;
        self.ip += 1;
        Ok(a as usize)
    }

    fn opcode(&mut self) -> Result<Opcode, Error> {
        let &instr = self.data.get(self.ip).ok_or(Error::InvalidAddr)?;
        // if instr
        self.ip += 1;
        let a = (instr / 10000) % 10;
        let b = (instr / 1000) % 10;
        let c = (instr / 100) % 10;
        // assert_eq!(a, 0, "third parameter must be in position mode!");
        if a != 0 {
            return Err(Error::InvalidMode);
        }
        if instr & 100 == 99 {
            return Ok(Opcode::Halt);
        }
        match instr % 10 {
            1 => Ok(Opcode::Add(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            2 => Ok(Opcode::Mul(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            3 => Ok(Opcode::Input(self.read_param(a)?)),
            4 => Ok(Opcode::Output(self.read_param(c)?)),
            5 => Ok(Opcode::Jnz(self.read_param(c)?, self.read_param(b)?)),
            6 => Ok(Opcode::Jz(self.read_param(c)?, self.read_param(b)?)),
            7 => Ok(Opcode::Lt(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            8 => Ok(Opcode::Eq(
                self.read_param(c)?,
                self.read_param(b)?,
                self.read_position()?,
            )),
            9 => Ok(Opcode::Offset(self.read_position()? as isize)),
            _ => Err(Error::InvalidInstr),
        }
    }

    fn fetch(&self, mode: Mode) -> Result<isize, Error> {
        match mode {
            Mode::Immediate(i) => Ok(i),
            Mode::Position(idx) => self.data.get(idx).copied().ok_or(Error::InvalidAddr),
            Mode::Relative(off) => {
                let base = usize::try_from(self.base as isize + off).expect(&format!(
                    "relative addr out of bounds: {}+{}",
                    self.base, off
                ));
                self.data.get(base).copied().ok_or(Error::InvalidAddr)
            }
        }
    }

    pub fn decompile(mut self) {
        while self.ip < self.data.len() {
            print!("{}: ", self.ip);
            match self.opcode() {
                Ok(Opcode::Jz(a, b)) => {
                    println!("{:?}", Opcode::Jz(a, b));
                    if self.fetch(a) == Ok(0) {
                        self.ip = self.fetch(b).unwrap() as usize;
                    }
                }
                Ok(Opcode::Jnz(a, b)) => {
                    println!("{:?}", Opcode::Jnz(a, b));
                    if self.fetch(a) != Ok(0) {
                        self.ip = self.fetch(b).unwrap() as usize;
                    }
                }
                Ok(Opcode::Offset(off)) => {
                    self.base = usize::try_from(self.base as isize + off).unwrap()
                }
                Ok(op) => println!("{:?}", op),
                _ => {
                    self.ip += 2;
                    break;
                }
            }
        }
    }

    fn store_or_extend(&mut self, loc: usize, data: isize) {
        if loc >= self.data.len() {
            self.data
                .extend(std::iter::repeat(0).take(2 * (loc - self.data.len())));
        }
    }

    pub fn run<I: Iterator<Item = isize>>(&mut self, mut input: I) -> Result<isize, Error> {
        self.ip = 0;
        self.base = 0;
        while self.ip < self.data.len() {
            let ip = self.ip;
            let op = self.opcode()?;

            print!("{:3}: ", ip);
            op.pretty_print(&self);
            match op {
                Opcode::Halt => {
                    return Err(Error::Halted);
                }
                Opcode::Add(a, b, c) => {
                    self.store_or_extend(c, self.fetch(a)? + self.fetch(b)?);
                }
                Opcode::Mul(a, b, c) => {
                    self.store_or_extend(c, self.fetch(a)? * self.fetch(b)?);
                }
                Opcode::Input(idx) => {
                    self.store_or_extend(
                        self.fetch(idx)? as usize,
                        input.next().ok_or(Error::NoInput)?,
                    );
                }
                Opcode::Output(mode) => {
                    return self.fetch(mode);
                    // if self.data.get(self.ip)? == &99 {
                    //     return Some(self.fetch(mo?de));
                    // } else {
                    //     return Some(self.fetch(mo?de));
                    // }
                }
                Opcode::Jnz(a, b) => {
                    if self.fetch(a)? != 0 {
                        self.ip = self.fetch(b)? as usize;
                    }
                }
                Opcode::Jz(a, b) => {
                    if self.fetch(a)? == 0 {
                        self.ip = self.fetch(b)? as usize;
                    }
                }
                Opcode::Lt(a, b, c) => {
                    self.store_or_extend(
                        c,
                        if self.fetch(a)? < self.fetch(b)? {
                            1
                        } else {
                            0
                        },
                    );
                }
                Opcode::Eq(a, b, c) => {
                    self.store_or_extend(
                        c,
                        if self.fetch(a)? == self.fetch(b)? {
                            1
                        } else {
                            0
                        },
                    );
                }
                Opcode::Offset(a) => {
                    self.base = usize::try_from(self.base as isize + a).unwrap();
                    dbg!(self.base);
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
            Ok(Opcode::Mul(Mode::Position(4), Mode::Immediate(3), 4))
        );
        assert_eq!(
            vm.opcode(),
            Ok(Opcode::Add(Mode::Immediate(100), Mode::Immediate(-1), 4))
        );
    }

    #[test]
    fn jump_test() {
        let ex = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(7)), Ok(999));
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(8)), Ok(1000));
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(9)), Ok(1001));
    }

    #[test]
    fn relative() {
        let ex = "109,19,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        // vm.clone().decompile();
        assert_eq!(vm.run(std::iter::repeat(0)), Err(Error::Halted));
        assert_eq!(vm.base, 19);

        let ex = "104,1125899906842624,99";
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(0)), Ok(1125899906842624));
    }

    #[test]
    fn jnz() {
        // set offset to 9
        // if 4 != 0 then jump to [base -1]
        // output cell 0
        let ex = "9,9,2105,4,-1,4,0,99,5";
        let mut vm = ex.parse::<Vm>().unwrap();
        assert_eq!(vm.run(std::iter::repeat(0)), Ok(9));
    }
}
