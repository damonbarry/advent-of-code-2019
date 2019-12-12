enum Operation {
    Addition { in1: usize, in2: usize, out: usize },
    Halt,
    Invalid,
}

pub struct IntcodeComputer {
    pub state: Vec<i64>,
    pos: usize,
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            state: Vec::new(),
            pos: 0,
        }
    }

    pub fn run(&mut self, program: impl Iterator<Item = i64>) {
        self.state = program.collect();

        let mut halt = false;

        while !halt {
            let op = self.next_op();
            match op {
                Operation::Halt => halt = true,
                Operation::Addition { in1, in2, out } => {
                    self.state[out] = self.state[in1] + self.state[in2];
                },
                Operation::Invalid => {},
            };
        }
    }

    fn next_op(&mut self) -> Operation {
        match self.state[self.pos] {
            1 => {
                self.pos += 4;
                Operation::Addition {
                    in1: self.state[self.pos - 3] as usize,
                    in2: self.state[self.pos - 2] as usize,
                    out: self.state[self.pos - 1] as usize,
                }
            },
            99 => {
                self.pos = 0;
                Operation::Halt
            },
            _ => Operation::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_first_opcode() {
        let input = std::fs::read_to_string("src/day02/input.txt").unwrap();
        let program: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        assert_eq!(1, program[0]);
    }

    #[test]
    fn computer_understands_halt() {
        let program = [99];
        let mut computer = IntcodeComputer::new();
        computer.run(program.iter().cloned());
        assert_eq!(&program[..], &computer.state[..]);
    }

    #[test]
    fn computer_understands_add() {
        let program = [1, 5, 6, 7, 99, 3, 7, 0];
        let mut computer = IntcodeComputer::new();
        computer.run(program.iter().cloned());
        assert_eq!(&[1, 5, 6, 7, 99, 3, 7, 10], &computer.state[..]);
    }
}
