enum Operation {
    Addition { in1: usize, in2: usize, out: usize },
    Halt,
    Invalid,
}

pub struct Error {
    pub position: usize,
}

pub struct Program {
    pub state: Vec<i64>,
}

impl Program {
    pub fn new(init: Vec<i64>) -> Program {
        Program {
            state: init,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.state.is_empty() {
            return Ok(());
        }

        let mut pos: usize = 0;

        loop {
            let op = self.next_op(&mut pos);
            match op {
                Operation::Halt => {
                    return Ok(());
                },
                Operation::Addition { in1, in2, out } => {
                    self.state[out] = self.state[in1] + self.state[in2];
                },
                Operation::Invalid => {
                    return Err(Error { position: pos });
                },
            };
        }
    }

    fn next_op(&mut self, pos: &mut usize) -> Operation {
        match self.state[*pos] {
            1 => {
                *pos += 4;
                Operation::Addition {
                    in1: self.state[*pos - 3] as usize,
                    in2: self.state[*pos - 2] as usize,
                    out: self.state[*pos - 1] as usize,
                }
            },
            99 => {
                *pos = 0;
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
    fn runs_an_empty_program() {
        let instructions = [];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&instructions[..], &program.state[..]);
    }

    #[test]
    fn fails_to_run_an_invalid_program() {
        let instructions = [7777].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
    }

    #[test]
    fn program_remembers_position_of_invalid_opcode() {
        let instructions = [1, 5, 6, 7, 12345, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_err());
        assert_eq!(4, result.unwrap_err().position);
    }

    #[test]
    fn understands_halt() {
        let instructions = [99];
        let mut program = Program::new(instructions.to_vec());
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&instructions[..], &program.state[..]);
    }

    #[test]
    fn understands_add() {
        let instructions = [1, 5, 6, 7, 99, 3, 7, 0].to_vec();
        let mut program = Program::new(instructions);
        let result = program.run();
        assert!(result.is_ok());
        assert_eq!(&[1, 5, 6, 7, 99, 3, 7, 10], &program.state[..]);
    }
}
